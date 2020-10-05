pub(crate) mod graph;

use serde::{Serialize, Deserialize};
use crate::data_structures::graph::Province;

/*
{
"Date_of_report":"2020-03-13 10:00:00",
"Municipality_code":"GM0003",
"Municipality_name":"Appingedam",
"Province":"Groningen",
"Total_reported":0,
"Hospital_admission":0,
"Deceased":0
}
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct AmountOfCasesPerTownshipCumulative {
    #[serde(alias = "Date_of_report")]
    date_of_report: String,
    #[serde(alias = "Municipality_code")]
    municipality_code: Option<String>,
    #[serde(alias = "Municipality_name")]
    municipality_name: Option<String>,
    #[serde(alias = "Province")]
    province: String,
    #[serde(alias = "Total_reported")]
    total_reported: u64,
    #[serde(alias = "Hospital_admission")]
    hospital_admission: u64,
    #[serde(alias = "Deceased")]
    deceased: u64
}

//{
// "Date_of_report":"2020-09-13 10:00:00",
// "Date_of_publication":"2020-02-27",
// "Municipality_code":"GM0003",
// "Municipality_name":"Appingedam",
// "Province":"Groningen",
// "Security_region_code":"VR01",
// "Security_region_name":"Groningen",
// "Municipal_health_service":"GGD Groningen",
// "ROAZ_region":"Acute Zorgnetwerk Noord Nederland",
// "Total_reported":0,
// "Hospital_admission":0,
// "Deceased":0
// }


#[derive(Serialize, Deserialize, Debug)]
pub struct AmountOfCasesPerTownshipPerDayRecord {
    #[serde(alias = "Date_of_report")]
    date_of_report: String,
    #[serde(alias = "Date_of_publication")]
    date_of_publication: String,
    #[serde(alias = "Municipality_code")]
    municipality_code: Option<String>,
    #[serde(alias = "Municipality_name")]
    municipality_name: Option<String>,
    #[serde(alias = "Province")]
    province: String,
    #[serde(alias = "Security_region_code")]
    security_region_code: Option<String>,
    #[serde(alias = "Security_region_name")]
    security_region_name: Option<String>,
    #[serde(alias = "Municipal_health_service")]
    municipal_health_service: String,
    #[serde(alias = "ROAZ_region")]
    roaz_region: Option<String>,
    #[serde(alias = "Total_reported")]
    total_reported: u64,
    #[serde(alias = "Hospital_admission")]
    hospital_admission: u64,
    #[serde(alias = "Deceased")]
    deceased: u64
}

//{
//    "Date_file":"2020-09-13 10:00:00",
//    "Date_statistics":"2020-01-01",
//    "Date_statistics_type":"DOO",
//    "Agegroup":"40-49",
//    "Sex":"Female",
//    "Province":"Noord-Holland",
//    "Hospital_admission":"No",
//    "Deceased":"No",
//    "Week_of_death":null,
//    "Municipal_health_service":"GGD Amsterdam"
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct NationalWideCases {
    #[serde(alias = "Date_file")]
    date_file: String,
    #[serde(alias = "Date_statistics")]
    date_statistics: String,
    #[serde(alias = "Date_statistics_type")]
    date_statistics_type: String,
    #[serde(alias = "Agegroup")]
    age_group: String,
    #[serde(alias = "Sex")]
    sex: String,
    #[serde(alias = "Province")]
    province: String,
    #[serde(alias = "Hospital_admission")]
    hospital_admission: String,
    #[serde(alias = "Deceased")]
    deceased: String,
    #[serde(alias = "Week_of_death")]
    security_region_name: Option<String>,
    #[serde(alias = "Municipal_health_service")]
    municipal_health_service: String
}

/*
{
    "Date":"2020-03-11",
    "prev_low":99822,
    "prev_avg":120072,
    "prev_up":138037,
    "POPULATION":"hosp"
 }
 */

#[derive(Serialize, Deserialize, Debug)]
pub struct Prevalence {
    #[serde(alias = "Date")]
    date: String,
    #[serde(alias = "prev_low")]
    prev_low: Option<u64>,
    #[serde(alias = "prev_avg")]
    prev_avg: Option<u64>,
    #[serde(alias = "prev_up")]
    prev_up: Option<u64>,
    #[serde(alias = "POPULATION")]
    population: String
    
}

/*
{
   "Date":"2020-04-28",
   "Rt_low":0.43,
   "Rt_avg":0.72,
   "Rt_up":1.03,
   "POPULATION":"hosp"
 }
 */

#[derive(Serialize, Deserialize, Debug)]
pub struct ReproductionNumber {
    #[serde(alias = "Date")]
    date: String,
    #[serde(alias = "Rt_low")]
    rt_low: Option<f32>,
    #[serde(alias = "Rt_avg")]
    rt_avg: Option<f32>,
    #[serde(alias = "Rt_up")]
    rt_up: Option<f32>,
    #[serde(alias = "POPULATION")]
    population: String
    
}

/*
{
   "Date_measurement": "2020-03-30",
   "RWZI_AWZI_code": 32002,
   "RWZI_AWZI_name": "Tilburg",
   "X_coordinate": 132554,
   "Y_coordinate": 401565,
   "Postal_code": "5048TD",
   "Security_region_code": "VR20",
   "Security_region_name": "Midden- en West-Brabant",
   "Percentage_in_security_region": "1",
   "RNA_per_ml": 1837,
   "Representative_measurement": true
 }
 */

#[derive(Serialize, Deserialize, Debug)]
pub struct SewageData
 {
    #[serde(alias = "Date_measurement")]
    date_measurement: String,
    #[serde(alias = "RWZI_AWZI_code")]
    rwzi_awzi_code: u64,
    #[serde(alias = "RWZI_AWZI_name")]
    rwzi_awzi_name: String,
    #[serde(alias = "X_coordinate")]
    x_coordinate: u64,
    #[serde(alias = "Y_coordinate")]
    y_coordinate: u64,
    #[serde(alias = "Postal_code")]
    postal_code: String,
    #[serde(alias = "Security_region_code")]
    security_region_code: String,
    #[serde(alias = "Security_region_name")]
    security_region_name: String,
    #[serde(alias = "Percentage_in_security_region")]
    percentage_in_security_region: String,
    #[serde(alias = "RNA_per_ml")]
    rna_per_ml: u64,
    #[serde(alias = "Representative_measurement")]
    representative_measurement: bool

    
}

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