use serde::{Serialize, Deserialize};

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
