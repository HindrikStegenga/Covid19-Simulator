mod data_structures;

pub use data_structures::*;

use serde::{Serialize, de::DeserializeOwned};
use std::io::Read;

use plotters::{*, prelude::*, drawing::*};


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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let file = match load_file::<Vec<AmountOfCasesPerTownshipPerDayRecord>>("./dataset/COVID-19_aantallen_gemeente_per_dag.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return Err("Could not load file".into()) }
    };
    
     println!("{}", std::env::current_dir().unwrap().display());
    let file = match load_file::<Vec<AmountOfCasesPerTownshipCumulative>>("./dataset/COVID-19_aantallen_gemeente_cumulatief.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return }
    };
    
     println!("{}", std::env::current_dir().unwrap().display());
    let file2 = match load_file::<Vec<NationalWideCases>>("./dataset/COVID-19_casus_landelijk.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
   // println!("{:#?}", file2);

    println!("{}", std::env::current_dir().unwrap().display());
    let file3 = match load_file::<Vec<Prevalence>>("./dataset/COVID-19_prevalentie.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
   // println!("{:#?}", file3);

    println!("{}", std::env::current_dir().unwrap().display());
    let file4 = match load_file::<Vec<ReproductionNumber>>("./dataset/COVID-19_reproductiegetal.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
   // println!("{:#?}", file4);

    println!("{}", std::env::current_dir().unwrap().display());
    let file5 = match load_file::<Vec<SewageData>>("./dataset/COVID-19_rioolwaterdata.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
   // println!("{:#?}", file5)


    let mut backend = BitMapBackend::new("./output/test.png", (800,600));
    let mut drawing_area = backend.into_drawing_area();

    drawing_area.fill(&WHITE);
    drawing_area = drawing_area.margin(50,50,50,50);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Test 2 Drawing",("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;


    let mut points : Vec<(f32, f32)> = generate_range(0.0,100.0, 0.1).into_iter().map(|c| (c,c)).collect();

    chart.draw_series(LineSeries::new(points, &RED))?;


    //println!("{:#?}", file);
    Ok(())
}

