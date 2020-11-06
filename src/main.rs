mod data_structures;
mod float_helper;

pub use float_helper::*;
pub use data_structures::*;

use serde::{Serialize, de::DeserializeOwned};
use std::io::Read;

use plotters::{*, prelude::*, drawing::*};
use crate::NonNanF32;

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
    for i in 0..steps {
        vec.push(start + (i as f32) * step)
    }
    vec
}

const TIMESPAN_IN_DAYS: usize = 400;
const INITIAL_POPULATION: u32 = 1_000;
const INITIAL_SPREADERS: u32 = 50;

const NATURAL_BIRTH_RATE: f32 = 0.011 / 365.0; // 6% growth a year
const NATURAL_DEATH_RATE: f32 = 0.005 / 365.0;

const DISEASE_PERIOD: usize = 14; // Time it takes for infected people to recover or die.
const INCUBATION_PERIOD: usize = 7; // Time it takes for exposed people to become sick + infectious.
const MORTALITY_RATE: f32 = 0.25; // Percentage of infected people who die.

const R_NAUGHT: f32 = 2.0; // 2.0 = t / (1.0/incubation_period)
const HOSPITALIZATION_RATE: f32 = 0.25; //Amount of recovering people ending up in hospital, thus counting towards max hospital cap.

const MAX_HOSPITAL_CAPACITY: f32 = 25.0; // Absolute amount of hospital capacity
const LOCKDOWN_TRIGGER_FRACTION: f32 = 0.6; // Fraction at which lockdown is imposed
const LOCKDOWN_LIFTING_FRACTION: f32 = 0.4; // Fraction at which lockdown is lifted
const LOCKDOWN_EFFECTIVENESS: f32 = 0.45; // Fraction as to how effective the lockdown is.

fn rate_of_change_with_time(previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> Vec<f32> {
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

    const PI : f32 = std::f32::consts::PI;

    let hospitalizations_delayed = previous_data[previous_data.len() - ((INCUBATION_PERIOD as f32 / h) as usize)][6];
    let population_delayed = previous_data[previous_data.len() - ((INCUBATION_PERIOD as f32 / h) as usize)][5];

    let mut measures_change = 0.0;
    if infected > population_delayed / 100.0 {
        measures_change += 0.1;
    }
    if hospitalizations_delayed >= 0.4 * MAX_HOSPITAL_CAPACITY {
        measures_change += 0.4;
    }
    if hospitalizations_delayed >= LOCKDOWN_TRIGGER_FRACTION * MAX_HOSPITAL_CAPACITY {
        measures_change += LOCKDOWN_EFFECTIVENESS;
    }

    // 2.0 = t / (1.0/incubation_period)
    let base_transmission_rate = R_NAUGHT * (1.0 / INCUBATION_PERIOD as f32);

    let mut transmission_rate = base_transmission_rate * (1.0 - measures_change); // Change of s to e
    let infectiousness_rate = 1.0 / (INCUBATION_PERIOD as f32); // Change of e to i
    let recovery_rate = 1.0 / (DISEASE_PERIOD as f32); // Change of i to r

    let mut dydx = vec![
        /*s*/ NATURAL_BIRTH_RATE * population - ((transmission_rate) * susceptible * (infected / population as f32)) - (NATURAL_DEATH_RATE * susceptible),
        /*e*/ (transmission_rate * susceptible * (infected / population as f32)) - infectiousness_rate * exposed - (NATURAL_DEATH_RATE * exposed),
        /*i*/ (infectiousness_rate * exposed) - (recovery_rate * infected) - (NATURAL_DEATH_RATE * infected),
        /*r*/ (recovery_rate * infected) * (1.0 - MORTALITY_RATE) - NATURAL_DEATH_RATE * recovered,
        /*d*/ (recovery_rate * infected) * MORTALITY_RATE,
        /*p*/ (NATURAL_BIRTH_RATE * population - NATURAL_DEATH_RATE * population) - ((recovery_rate * infected) * MORTALITY_RATE),
        /*h*/ ((infectiousness_rate * exposed) - (recovery_rate * infected) - (NATURAL_DEATH_RATE * infected)) * HOSPITALIZATION_RATE,
    ];

    // Adjust for time step h for the integrator
    for v in &mut dydx {
        *v *= h;
    }

    dydx
}

#[derive(Debug, Copy, Clone)]
pub struct InitialValue {
    value: f32,
    repeating_before: f32
}

/// Implements a Runge Kutta integrator.
fn rk4(t0: Vec<InitialValue>, step_size: f32, total_days: u32, f: fn(&Vec<f32>, &[Vec<f32>],  f32, f32) -> Vec<f32>) -> Vec<Vec<f32>> {
    let initial_zero_values = ((DISEASE_PERIOD as f32 / step_size) as usize) + 1;
    let mut results = vec![t0.iter().map(|i| i.repeating_before).collect(); initial_zero_values];
    results.push(t0.iter().map(|i| i.value).collect());
    let iterations = f32::floor(total_days as f32 / step_size) as usize;
    for i in 0..iterations-1 {
        let (pr, last) = results.split_at(results.len() - 1);
        results.push(rk4_impl(last.first().unwrap(), pr,i as f32 * step_size, step_size, f));
    }

    results.split_at(initial_zero_values).1.to_vec()
}

fn rk4_impl(value:&Vec<f32>, previous_data: &[Vec<f32>], t: f32, h: f32, f: fn(&Vec<f32>, &[Vec<f32>], f32, f32)->Vec<f32>) -> Vec<f32> {
    let k1: Vec<f32> = f(value, previous_data, t, h).iter().map(|e|e*h).collect();
    let k2: Vec<f32> = f(&value.iter().enumerate().map(|(idx, e)| e + 0.5 * k1[idx]).collect(), previous_data, t + 0.5 * h, h).iter().map(|e|e*h).collect();
    let k3: Vec<f32> = f(&value.iter().enumerate().map(|(idx, e)| e + 0.5 * k2[idx]).collect(), previous_data, t + 0.5 * h, h).iter().map(|e|e*h).collect();
    let k4: Vec<f32> = f(&value.iter().enumerate().map(|(idx, e)| e + k3[idx]).collect(), previous_data, t + h, h).iter().map(|e|e*h).collect();
    return value.iter().enumerate().map(|(idx,e)| e + {(1.0/6.0) * (k1[idx] + 2.0 * k2[idx] + 2.0 * k3[idx] + k4[idx])}).collect();
}

predefined_color!(ORANGE, 255, 165, 0, "The predefined orange color");


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let t0 : Vec<InitialValue> = vec![
        InitialValue { value: (INITIAL_POPULATION - INITIAL_SPREADERS) as f32, repeating_before: 0.0 }, //Susceptible people
        InitialValue { value: INITIAL_SPREADERS as f32, repeating_before: 0.0 }, //Exposed people
        InitialValue { value: 0.0 as f32, repeating_before: 0.0 }, //Infected people
        InitialValue { value: 0.0, repeating_before: 0.0 }, //Recovered people
        InitialValue { value: 0.0, repeating_before: 0.0 }, //Dead people
        InitialValue { value: INITIAL_POPULATION as f32, repeating_before: INITIAL_POPULATION as f32}, // Population
        InitialValue {value: 0.0, repeating_before: 0.0}, // Hospitalizations
    ];
    let t0len = t0.len();

    let mut values = rk4(t0, 0.1, TIMESPAN_IN_DAYS as u32, rate_of_change_with_time);

    //values.iter_mut().map(|e|&mut e[6]).for_each(|mut e| *e = *e * 2000.0 + (INITIAL_POPULATION as f32 / 2.0));

    let max_pop : f32 = values.iter().map(|v| NonNanF32(v[5])).max().unwrap().0;

    let mut backend = BitMapBackend::new("./output/test.png", (1680,1440));
    let mut drawing_area = backend.into_drawing_area();

    drawing_area.fill(&WHITE);
    drawing_area = drawing_area.margin(50,50,50,50);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(&format!("SEIRD - R0: {:.1} - Recovery in days: {:.1} - Mortality: {:.2}", R_NAUGHT, DISEASE_PERIOD, MORTALITY_RATE), ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(20)
        //.build_cartesian_2d(0f32..TIMESPAN_IN_DAYS as f32, 0f32..1.0)?;
        .build_cartesian_2d(0f32..TIMESPAN_IN_DAYS as f32, 0f32..(max_pop + 0.1 * max_pop))?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let colors = [&ORANGE, &MAGENTA, &RED, &GREEN, &BLACK, &BLUE, &CYAN, &YELLOW];
    let labels = ["Susceptible", "Exposed", "Infected", "Recovered", "Deaths", "Population", "Hospitalizations", "Measures"];
    for idx in 0..t0len {

        let mut points : Vec<(f32, f32)> = generate_range(0.0, TIMESPAN_IN_DAYS as f32, 0.1).into_iter().enumerate().map(|(i, c)| (c, values[i][idx])).collect();

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

    //println!("{:#?}", values);
    Ok(())
}
