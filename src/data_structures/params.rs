
pub type MeasureFn = dyn Fn(&SimulationParameters, &Vec<f32>, &[Vec<f32>], f32, f32) -> f32;


/// Hand washing measure triggers at infected > 1% of population
fn hand_washing(parameters: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> f32 {

    let delayed_population = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][5];
    let delayed_infected = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][2];

    return if delayed_infected > (delayed_population / 100.0) {
        0.1
    } else { 0.0 };
}

/// Social distancing reduces transmission by having more distance between people and limits visits etc.
fn social_distancing(parameters: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> f32 {

    return 0.0
}





pub struct SimulationParameters {
    pub time_span_in_days: usize,
    pub initial_population: usize,
    pub initial_spreaders: usize,
    pub natural_birth_rate: f32,
    pub natural_death_rate: f32,
    pub sickness_period_in_days: usize,
    pub incubation_period_in_days: usize,
    pub mortality_rate: f32,
    pub r_naught: f32,
    pub hospitalization_rate: f32,
    pub max_hospital_capacity: usize,
    pub measures: Vec<Box<MeasureFn>>
}