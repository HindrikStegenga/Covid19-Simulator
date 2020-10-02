mod data_structures;

pub use data_structures::*;

use serde::{Serialize, de::DeserializeOwned};
use std::io::Read;

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


fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let file = match load_file::<Vec<AmountOfCasesPerTownshipCumulative>>("./dataset/COVID-19_aantallen_gemeente_cumulatief.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return }
    };
     println!("{:#?}", file);

    println!("{}", std::env::current_dir().unwrap().display());
    let file1 = match load_file::<Vec<AmountOfCasesPerTownshipPerDayRecord>>("./dataset/COVID-19_aantallen_gemeente_per_dag.json") {
        Some(v) => v,
        None => { println!("Could not load file!"); return }
    };
     println!("{:#?}", file1);
    
    println!("{}", std::env::current_dir().unwrap().display());
    let file2 = match load_file::<Vec<NationalWideCases>>("./dataset/COVID-19_casus_landelijk.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
    println!("{:#?}", file2);

    println!("{}", std::env::current_dir().unwrap().display());
    let file3 = match load_file::<Vec<Prevalence>>("./dataset/COVID-19_prevalentie.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
    println!("{:#?}", file3);

    println!("{}", std::env::current_dir().unwrap().display());
    let file4 = match load_file::<Vec<ReproductionNumber>>("./dataset/COVID-19_reproductiegetal.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
    println!("{:#?}", file4);

    println!("{}", std::env::current_dir().unwrap().display());
    let file5 = match load_file::<Vec<SewageData>>("./dataset/COVID-19_rioolwaterdata.json") {
       Some(v) => v,
       None => { println!("Could not load file!"); return }
       };
    println!("{:#?}", file5)

}

