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

const TIMESPAN_IN_DAYS: usize = 1000;
const INFECTION_RATE: f32 = 1.1;
const RECOVERY_RATE: f32 = 0.0714; //People recover on average in 14 days

const INITIAL_POPULATION: u32 = 1000;
const INITIAL_SPREADERS: u32 = 1;

const DEATH_CHANCE: f32 = 0.8;
const TIME_DELAY_RECOVERY: usize = 14;
const BIRTH_RATE: f32 = 0.011;
const DEATH_RATE: f32 = 0.006;

fn rate_of_change_with_time(previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> Vec<f32> {
    // S(t) is susceptible
    // I(t) is infected
    // R(t) is recovered
    // D(t) is deaths


    // P is population
    // s(t) = S(t) / P
    // i(t) = I(t) / P
    // r(t) = R(t) / P
    // d(t) = D(t) / P


    // Susceptible equation
    // ds/dt = -b s(t) * i(t)

    // Infected equation
    // di/dt = b s(t) i(t) - k i(t)

    // Recovered equation
    // dr/dt = k i(t) * (1.0 - d)

    // Death equation
    // dd/dt = k i(t) * d


    //k is recovery period.
    //b is rate of infection.
    //d is chance of dying once infected.

    let susceptible = previous[0];
    let infected = previous[1];
    let recovered = previous[2];
    let population = previous[4];
    let died = previous[3];

    //let delta_susceptible = previous_data[previous_data.len() - ((TIME_DELAY_RECOVERY as f32 / h) as usize)][0];
    //let delta_infected = previous_data[previous_data.len() - ((TIME_DELAY_RECOVERY as f32 / h) as usize)][1];
    //let delta_recovered = previous_data[previous_data.len() - ((TIME_DELAY_RECOVERY as f32 / h) as usize)][2];

    let mut dydx = vec![
        /*s*/ BIRTH_RATE * population - (((INFECTION_RATE) * susceptible * infected) / population as f32) - DEATH_RATE * susceptible,
        /*i*/ (((INFECTION_RATE * susceptible * infected) / population as f32) - RECOVERY_RATE * infected) - DEATH_RATE * infected,
        /*r*/ (RECOVERY_RATE * infected) * (1.0 - DEATH_CHANCE) - DEATH_RATE * recovered,
        /*d*/ (RECOVERY_RATE * infected) * DEATH_CHANCE,
        /*p*/ (BIRTH_RATE * population - DEATH_RATE * population) - ((RECOVERY_RATE * infected) * DEATH_CHANCE)
    ];

    // Adjust for actual population values and time step h for the integrator
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
    let initial_zero_values = ((TIME_DELAY_RECOVERY as f32 / step_size) as usize) + 1;
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
    /*
    println!("{}", std::env::current_dir().unwrap().display());

    let file = match load_file::<Vec<ProvinceData>>("./dataset/provinces.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    let graph = ProvinceGraph::from(file);
    //println!("{:#?}", graph);

    let file = match load_file::<Vec<AmountOfCasesPerTownshipPerDayRecord>>("./dataset/COVID-19_aantallen_gemeente_per_dag.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };

    let file = match load_file::<Vec<AmountOfCasesPerTownshipCumulative>>("./dataset/COVID-19_aantallen_gemeente_cumulatief.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };

    let file2 = match load_file::<Vec<NationalWideCases>>("./dataset/COVID-19_casus_landelijk.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    // println!("{:#?}", file2);

    let file3 = match load_file::<Vec<Prevalence>>("./dataset/COVID-19_prevalentie.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    // println!("{:#?}", file3);

    let file4 = match load_file::<Vec<ReproductionNumber>>("./dataset/COVID-19_reproductiegetal.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    // println!("{:#?}", file4);

    let file5 = match load_file::<Vec<SewageData>>("./dataset/COVID-19_rioolwaterdata.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    // println!("{:#?}", file5)
    */

    let t0 : Vec<InitialValue> = vec![
        InitialValue { value: (INITIAL_POPULATION - INITIAL_SPREADERS) as f32, repeating_before: 0.0 }, //Susceptible people
        InitialValue { value: INITIAL_SPREADERS as f32, repeating_before: 0.0 }, //Infected people
        InitialValue { value: 0.0, repeating_before: 0.0 }, //Recovered people,
        InitialValue { value: 0.0, repeating_before: 0.0 }, //Dead people
        InitialValue { value: INITIAL_POPULATION as f32, repeating_before: INITIAL_POPULATION as f32}
    ];
    let t0len = t0.len();

    let mut values = rk4(t0, 0.1, TIMESPAN_IN_DAYS as u32, rate_of_change_with_time);

    let max_pop : f32 = values.last().unwrap().iter().map(|f| NonNanF32::new(*f) ).max().unwrap().unwrap().0;

    let mut backend = BitMapBackend::new("./output/test.png", (1680,1440));
    let mut drawing_area = backend.into_drawing_area();

    drawing_area.fill(&WHITE);
    drawing_area = drawing_area.margin(50,50,50,50);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(&format!("SIR - Infection rate: {:.1} - Recovery in days: {:.1} - Mortality: {:.2}", INFECTION_RATE, (1.0 / RECOVERY_RATE), DEATH_CHANCE),("sans-serif", 40).into_font())
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

    let colors = [&ORANGE, &RED, &GREEN, &BLACK, &BLUE];
    let labels = ["Susceptible", "Infected", "Recovered", "Dead", "Population"];
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
