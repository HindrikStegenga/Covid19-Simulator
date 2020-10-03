mod data_structures;

pub use data_structures::*;

use serde::{Serialize, de::DeserializeOwned};
use std::io::Read;

use plotters::{*, prelude::*, drawing::*};
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

// sick_people(t)
// dy/dx sick_people(t)

// ^
// RK 4 (45)
// ^

// dy/dx sick_people(t)
// d2y/d2x sick_people(t)

// sick_people(t = 0) = 10
// total_people(t = 0) = 1000
// infection_rate = 1.1 per day
// cure_rate = 0.9
// death_rate = 0.1
// 100.0 / 14 = 7.14% //people recover/die in 14 days on average.

// dy/dx sp(t) = sp(t) * infection_rate - 0.0714 * sp(t)

//POPULATION=1000
//no of sic <= POPULATION
//excluded_ppl become immune/ die
//sick+exc+S=1000
//POPULATION-(sick+exc) then infect =0
//S' = -aSI
//R'= aSI - bI
//I' = bI
//14 days 


//I = infected people, S = susceptible people, P = population, R = recovered/dead people, F = base rate of infection / per day per person.

// I(t+1) = P - S * I(t) * F
// S = P - (R + I)
// R = ?
// F = infection rate

//I(t+1)
//S(t+1) =
//R(t+1)

//t(0) => S = P - I
//t(0) => I = 3


//I(t)' = I(t) * f? - (healing * I(t))
//I(t)'' = function of F, S, I and P
//I(t)'' = I(t)' > S(t) or not. S(t) < I(t') =>


//S(t) = P - (R(t) + I(I))


//S(t)' = P - S(t) * I(t) - R(t)
//R(t)' = 0.0714 * I(t)

const TIME: usize = 1000;
const BASE_INFECTION_RATE : f32 = 0.1;
const BASE_RECOVERY_RATE : f32 = 0.0714; // every 14 days a person recovers on average
const POPULATION: f32 = 1000.0;
const DEATH_CHANCE: f32 = 0.2;

fn rate_of_change_with_time(previous: &Vec<f32>, time: f32, h: f32) -> Vec<f32> {
    // I
    // R
    let infected = previous[0];
    let recovered = previous[1];
    let susceptible = POPULATION - (infected + recovered);


    vec![
        // dy/dx I
        // dy/dx R
        f32::min(infected, susceptible) * (BASE_INFECTION_RATE * h),
        infected * (BASE_RECOVERY_RATE * h)
    ]
}

fn rk4(t0: Vec<f32>, step_size: f32, total_days: u32, f: fn(&Vec<f32>, f32, f32) ->Vec<f32>) -> Vec<Vec<f32>> {
    let mut results = vec![t0];
    let iterations = f32::floor(total_days as f32 / step_size) as usize;
    for i in 0..iterations-1 {
        results.push(rk4_impl(results.last().unwrap(), i as f32 * step_size, step_size, f));
    }

    results
}

fn rk4_impl(value:&Vec<f32>, t: f32, h: f32, f: fn(&Vec<f32>,f32,f32)->Vec<f32>) -> Vec<f32> {
    let k1: Vec<f32> = f(value, t, h).iter().map(|e|e*h).collect();
    let k2: Vec<f32> = f(&value.iter().enumerate().map(|(idx, e)| e + 0.5 * k1[idx]).collect(), t + 0.5 * h, h).iter().map(|e|e*h).collect();
    let k3: Vec<f32> = f(&value.iter().enumerate().map(|(idx, e)| e + 0.5 * k2[idx]).collect(), t + 0.5 * h, h).iter().map(|e|e*h).collect();
    let k4: Vec<f32> = f(&value.iter().enumerate().map(|(idx, e)| e + k3[idx]).collect(), t + h, h).iter().map(|e|e*h).collect();
    return value.iter().enumerate().map(|(idx,e)| e + {(1.0/6.0) * (k1[idx] + 2.0 * k2[idx] + 2.0 * k3[idx] + k4[idx])}).collect();
}

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

    let mut backend = BitMapBackend::new("./output/test.png", (1024,800));
    let mut drawing_area = backend.into_drawing_area();

    drawing_area.fill(&WHITE);
    drawing_area = drawing_area.margin(50,50,50,50);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Test 2 Drawing",("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(0f32..TIME as f32, -250f32..(POPULATION + 0.1 * POPULATION))?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let t0 : Vec<f32> = vec![1.0, 0.0];
    let mut values = rk4(t0.clone(), 0.1, TIME as u32, rate_of_change_with_time);
    values.iter_mut().for_each(|e| e.push(POPULATION - (e[0] + e[1])));
    let colors = [&RED, &BLACK, &BLUE];
    let labels = ["Infected", "Recovered/Dead", "Susceptible"];
    for idx in 0..(t0.len() + 1) {

        let mut points : Vec<(f32, f32)> = generate_range(0.0,TIME as f32, 0.1).into_iter().enumerate().map(|(i, c)| (c, values[i][idx])).collect();

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
