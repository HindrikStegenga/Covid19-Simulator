pub mod graph;
pub mod params;

pub use graph::*;
pub use params::*;

use serde::{Serialize, Deserialize};

/*
  {
    "name": "Groningen",
    "population": 583990,
    "density_per_square_km": 194,
    "connected_provinces": ["Friesland", "Drenthe"]
  }
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvinceData {
    name: String,
    population: u32,
    density_per_square_km: u16,
    connected_provinces: Vec<String>
}