mod data_structures;
mod float_helper;

use crate::graph::*;
pub use float_helper::*;
pub use data_structures::*;

use serde::{Serialize, de::DeserializeOwned};
use std::io::Read;

use plotters::{*, prelude::*, drawing::*};
use crate::NonNanF32;
use crate::params::SimulationParameters;
use crate::data_structures::params::hand_washing;

macro_rules! predefined_color {
    ($name:ident, $r:expr, $g:expr, $b:expr, $doc:expr) => {
        #[doc = $doc]
        pub const $name: RGBColor = RGBColor($r, $g, $b);
    };

    ($name:ident, $r:expr, $g:expr, $b:expr, $a: expr, $doc:expr) => {
        #[doc = $doc]
        pub const $name: RGBAColor = RGBAColor($r, $g, $b, $a);
    }
}


//use crate::graph::ProvinceGraph;


pub fn load_file_bin(path: &str) -> Option<Vec<u8>> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;
    Some(bytes)
}

pub fn load_file<T: DeserializeOwned>(path: &str) -> Option<T> {
    let data = load_file_bin(path)?;
    let obj: T = match serde_json::from_slice(data.as_slice()) {
        Ok(v) => v,
        Err(e) => {
            println!("{}",e);
            return None
        }
    };
    Some(obj)
}

fn generate_range(start: f32, end: f32, step: f32) -> Vec<f32> {
    assert!(end > start, "End must be larger than start");
    assert!(step > 0.0, "Step must be larger than 0.0");
    let steps = f32::ceil((end - start) / step) as usize;
    let mut vec = Vec::with_capacity(steps);
    for i in 0..steps+1 {
        vec.push(start + (i as f32) * step)
    }
    vec
}

fn generate_range_from_input(input_len: usize, step: f32) -> Vec<f32> {
    let mut vec = Vec::with_capacity(input_len);
    for i in 0..input_len {
        vec.push(0.0 + (i as f32) * step)
    }
    vec
}

const TIMESPAN_IN_DAYS: usize = 2 * 365;
const INITIAL_SPREADERS: u32 = 50;

const NATURAL_BIRTH_RATE: f32 = 0.011 / 365.0; // 6% growth a year
const NATURAL_DEATH_RATE: f32 = 0.005 / 365.0;

const DISEASE_PERIOD: usize = 7; // Time it takes for infected people to recover or die.
const INCUBATION_PERIOD: usize = 7; // Time it takes for exposed people to become sick + infectious.
const MORTALITY_RATE: f32 = 0.25; // Percentage of infected people who die.
const IMMUNITY_WANING_TIME_IN_DAYS: usize = 30 * 4; // Immunity wanes after 4 months

// R0 = beta / gamma
const R_NAUGHT: f32 = 2.0;

const HOSPITALIZATION_RATE: f32 = 0.25; //Amount of recovering people ending up in hospital, thus counting towards max hospital cap.
const MAX_HOSPITAL_CAPACITY: usize = 25; // Absolute amount of hospital capacity

const ENABLE_TRAFFIC: bool = true;
const TRAFFIC_RATE: f32 = 0.0001; // Percentage of E which travels to other places

fn rate_of_change_with_time(sp: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> Vec<f32> {
    // S(t) is susceptible
    // E(t) is exposed
    // I(t) is infectious/infected
    // R(t) is recovered
    // D(t) is deaths
    // P(t) is population

    // Susceptible equation
    // ds/dt = b * P - d * s(t) - b * s(t) * i(t) / P

    // Exposed equation
    // de/dt = b * s(t) * i(t) / P - d * e(t) - q * e(t)

    // Infected equation
    // di/dt = q * e(t) - r * i(t) - d * i(t)

    // Recovered equation
    // dr/dt = r * i(t) * (1.0 - k) - (d * r(t))

    // Deaths equation
    // dd/dt = r * i(t) * k

    // Population equation
    // dp/dt = (b * P - d * P) - (r * i(t) * k)

    // b is natural birth rate
    // d is natural death rate
    // b is transmission rate
    // q is incubation rate. (1 / incubation time)
    // r is recovery rate of the disease
    // k is fatality chance of the disease

    // In our system people die at a rate identical to the recovery rate, hence we multiply the recovery rate as 1 - mortality,
    // but subtract the whole set of recovered from infectious.

    let susceptible = previous[0];
    let exposed = previous[1];
    let infected = previous[2];
    let recovered = previous[3];
    // let dead = previous[4];
    let population = previous[5];
    //let hospitalizations = previous[6];

    let mut measures_change = 0.0;
    for measure in &sp.measures {
        measures_change += measure(&sp, previous, previous_data, time, h);
    }

    let recovery_rate = 1.0 / (sp.sickness_period_in_days as f32); // Change of i to r
    let base_infection_rate = sp.r_naught * recovery_rate as f32;
    let mut infection_rate = base_infection_rate * (1.0 - measures_change); // Change of s to e
    let incubation_rate = 1.0 / (sp.incubation_period_in_days as f32); // Change of e to i
    let immunity_waning_rate = 1.0 / (sp.immunity_waning_period_in_days as f32);

    let mut dydx = vec![
        /*s*/ sp.natural_birth_rate * population - ((infection_rate) * susceptible * (infected / population as f32)) - (sp.natural_death_rate * susceptible) + (immunity_waning_rate * recovered),
        /*e*/ (infection_rate * susceptible * (infected / population as f32)) - incubation_rate * exposed - (sp.natural_death_rate * exposed),
        /*i*/ (incubation_rate * exposed) - (recovery_rate * infected) - (sp.natural_death_rate * infected),
        /*r*/ (recovery_rate * infected) * (1.0 - sp.mortality_rate) - sp.natural_death_rate * recovered - (immunity_waning_rate * recovered),
        /*d*/ (recovery_rate * infected) * sp.mortality_rate + sp.natural_death_rate * susceptible + sp.natural_death_rate * exposed + sp.natural_death_rate * infected + sp.natural_death_rate * recovered,
        /*p*/ (sp.natural_birth_rate * population - sp.natural_death_rate * population) - ((recovery_rate * infected) * sp.mortality_rate),
        /*h*/ ((incubation_rate * exposed) - (recovery_rate * infected) - (sp.natural_death_rate * infected)) * sp.hospitalization_rate,
    ];
    dydx
}

#[derive(Debug, Copy, Clone)]
pub struct InitialValue {
    value: f32,
    repeating_before: f32
}

fn rk4_impl(value: &Vec<f32>, previous_data: &[Vec<f32>], t: f32, h: f32, params: &SimulationParameters, f: fn(&SimulationParameters, &Vec<f32>, &[Vec<f32>], f32, f32)->Vec<f32>) -> Vec<f32> {
    let k1: Vec<f32> = f(params, value, previous_data, t, h).iter().map(|e|e*h).collect();
    let k2: Vec<f32> = f(params, &value.iter().enumerate().map(|(idx, e)| e + 0.5 * k1[idx]).collect(), previous_data, t + 0.5 * h, h).iter().map(|e|e*h).collect();
    let k3: Vec<f32> = f(params, &value.iter().enumerate().map(|(idx, e)| e + 0.5 * k2[idx]).collect(), previous_data, t + 0.5 * h, h).iter().map(|e|e*h).collect();
    let k4: Vec<f32> = f(params, &value.iter().enumerate().map(|(idx, e)| e + k3[idx]).collect(), previous_data, t + h, h).iter().map(|e|e*h).collect();
    return value.iter().enumerate().map(|(idx,e)| e + {(1.0/6.0) * (k1[idx] + 2.0 * k2[idx] + 2.0 * k3[idx] + k4[idx])}).collect();
}

predefined_color!(ORANGE, 255, 165, 0, "The predefined orange color");


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file = match load_file::<Vec<ProvinceData>>("./dataset/provinces.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    //println!("{:#?}", file);

    let graph = ProvinceGraph::from(file);

    let mut province_parameters: Vec<SimulationParameters> = vec![];
    let mut initial_values: Vec<Vec<InitialValue>> = vec![];
    let mut results: Vec<Vec<Vec<f32>>> = vec![];

    // Step size to use in the simulation.
    let step_size = 0.1;

    let mut mean_density = 0.0f32;
    for p in &graph {
        mean_density += p.density_per_square_km as f32;
    }
    mean_density /= graph.len() as f32;

    for province in &graph {
        // Compute relative change compared to mean density. Used to adjust infection rate.
        let relative_change: f32 = (province.density_per_square_km as f32 - mean_density) / mean_density;

        // Set up all parameters of the simulation
        let parameters = SimulationParameters {
            time_span_in_days: TIMESPAN_IN_DAYS,
            initial_population: province.population as usize,
            initial_spreaders:  if province.name == "Noord-Brabant" { INITIAL_SPREADERS as usize } else { 0 },
            natural_birth_rate: NATURAL_BIRTH_RATE, 
            natural_death_rate: NATURAL_DEATH_RATE,
            sickness_period_in_days: DISEASE_PERIOD,
            incubation_period_in_days: INCUBATION_PERIOD,
            immunity_waning_period_in_days: IMMUNITY_WANING_TIME_IN_DAYS,
            mortality_rate: MORTALITY_RATE,
            r_naught: R_NAUGHT * (1.0 + relative_change),
            hospitalization_rate: HOSPITALIZATION_RATE,
            max_hospital_capacity: MAX_HOSPITAL_CAPACITY,
            traffic_rate: TRAFFIC_RATE,
            measures: vec![
                //Box::from(hand_washing)
            ]
        };

        // Set up initial values for the system. repating_before are for when DDE's are used.
        let t0 : Vec<InitialValue> = vec![
            InitialValue { value: (parameters.initial_population - parameters.initial_spreaders) as f32, repeating_before: 0.0 }, //Susceptible people
            InitialValue { value: parameters.initial_spreaders as f32, repeating_before: 0.0 }, //Exposed people
            InitialValue { value: 0.0 as f32, repeating_before: 0.0 }, //Infected people
            InitialValue { value: 0.0, repeating_before: 0.0 }, //Recovered people
            InitialValue { value: 0.0, repeating_before: 0.0 }, //Dead people
            InitialValue { value: parameters.initial_population as f32, repeating_before: parameters.initial_population as f32}, // Population
            InitialValue {value: 0.0, repeating_before: 0.0}, // Hospitalizations
        ];

        let initial_zero_values = ((parameters.incubation_period_in_days as f32 / step_size) as usize) + 1;
        let mut province_results = vec![t0.iter().map(|i| i.repeating_before).collect(); initial_zero_values];
        province_results.push(t0.iter().map(|i| i.value).collect());

        results.push(province_results);
        province_parameters.push(parameters);
        initial_values.push(t0);
    }

    println!("Simulation in progress...");

    // Execute iterations
    let iterations = f32::floor(TIMESPAN_IN_DAYS as f32 / step_size) as usize;
    for i in 0..iterations-1 {
        for province_idx in 0..province_parameters.len() {
            let (pr, last) = results[province_idx].split_at(results[province_idx].len() - 1);
            let new_step = rk4_impl(last.first().unwrap(), pr,i as f32 * step_size, step_size, &province_parameters[province_idx], rate_of_change_with_time);
            results[province_idx].push(new_step);
        }

        // This part is responsible for computing traffic between provinces.
        if ENABLE_TRAFFIC {

            // Effectively turns a few susceptible people in other provinces into exposed.
            for province_idx in 0..province_parameters.len() {
                let connected_count = graph[province_idx].connected_provinces.len();
                let province_e = results[province_idx].last().unwrap()[1];
                let delta_e = (province_parameters[province_idx].traffic_rate * province_e * step_size);


                for idx in 0..connected_count {
                    let connected_idx = graph[province_idx].connected_provinces[idx];
                    if results[connected_idx].last_mut().unwrap()[0] > (delta_e / connected_count as f32) {
                        results[connected_idx].last_mut().unwrap()[0] -= (delta_e / connected_count as f32);
                        results[connected_idx].last_mut().unwrap()[1] += (delta_e / connected_count as f32);
                    }
                }
            }
        }
    }

    println!("Simulation done. Generating graphs...");

    // Show results
    for province_idx in 0..province_parameters.len() {
        let initial_zero_values = ((province_parameters[province_idx].sickness_period_in_days as f32 / step_size) as usize) + 1;
        results[province_idx] = results[province_idx].split_at(initial_zero_values).1.to_vec();

        draw(&graph[province_idx].name, &initial_values[province_idx], &province_parameters[province_idx], &results[province_idx], step_size)?;
    }

    Ok(())
}

// This function is responsible for plotting the data onto a 2D graph.
fn draw(output_file_name: &str, t0: &Vec<InitialValue>, parameters: &SimulationParameters, rk4_results: &Vec<Vec<f32>>, step_size: f32) -> Result<(), Box<dyn std::error::Error>> {
    
    let max_pop : f32 = rk4_results.iter().map(|v| NonNanF32(v[2])).max().unwrap().0;
    
    let var = String::from(String::from("./output/") + output_file_name) + ".png";
    let mut backend = BitMapBackend::new(&var, (800,800));
    let mut drawing_area = backend.into_drawing_area();

    drawing_area.fill(&WHITE);
    drawing_area = drawing_area.margin(50,50,50,50);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(&format!("SEIRD - Infection rate: {:.1} - Recovery in days: {:.1} - Mortality: {:.2}", parameters.r_naught, parameters.sickness_period_in_days, parameters.mortality_rate), ("sans-serif", 16).into_font())
        .x_label_area_size(20)
        .y_label_area_size(20)
        //.build_cartesian_2d(0f32..TIMESPAN_IN_DAYS as f32, 0f32..1.0)?;
        .build_cartesian_2d(0f32..parameters.time_span_in_days as f32, 0f32..(max_pop + 0.1 * max_pop))?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_label_formatter(&|x| format!("{:.0}", x))
        .draw()?;

    let colors = [&ORANGE, &MAGENTA, &RED, &GREEN, &BLACK, &BLUE, &CYAN, &YELLOW];
    let labels = ["Susceptible", "Exposed", "Infected", "Recovered", "Deaths", "Population", "Hospitalizations", "Measures"];
    for idx in 0..t0.len() {

        let mut points : Vec<(f32, f32)> = generate_range_from_input(rk4_results.len(), step_size).into_iter().enumerate().map(|(i, c)| (c, rk4_results[i][idx])).collect();

        chart.draw_series(LineSeries::new(points, colors[idx]))?
            .label(labels[idx])
            .legend( move |(x, y)|
                    PathElement::new(vec![(x, y), (x + 20, y)], colors[idx])
            );
    }
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
