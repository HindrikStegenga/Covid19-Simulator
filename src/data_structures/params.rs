
pub type MeasureFn = dyn Fn(&SimulationParameters, &Vec<f32>, &[Vec<f32>], f32, f32) -> f32;


/// Hand washing measure triggers at infected > 1% of population
pub fn hand_washing(parameters: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> f32 {

    let delayed_population = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][5];
    let delayed_infected = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][2];

    return if delayed_infected > (delayed_population / 100.0) {
        0.1
    } else { 0.0 };
}

/// Social distancing reduces transmission by having more distance between people and limits visits etc.
pub fn social_distancing(parameters: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> f32 {
    let delayed_hospitalizations = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][6];
    return if delayed_hospitalizations >= 0.1 * parameters.max_hospital_capacity as f32 {
        return 0.1
    } else { 0.0 }
}

/// Soft lock down is triggered based on hospital capacity. It reduces transmissions of disease quite a bit
pub fn soft_lock_down(parameters: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> f32 {
    let delayed_hospitalizations = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][6];

    return if delayed_hospitalizations >= 0.3 * parameters.max_hospital_capacity as f32{
        0.4
    } else { 0.0 }
}

/// Hard lock down is triggered based on hospital capacity. It reduces transmissions of disease significantly.
pub fn hard_lock_down(parameters: &SimulationParameters, previous: &Vec<f32>, previous_data: &[Vec<f32>], time: f32, h: f32) -> f32 {

    let delayed_hospitalizations = previous_data[previous_data.len() - ((parameters.incubation_period_in_days as f32 / h) as usize)][6];

    return if delayed_hospitalizations >= 0.6 * parameters.max_hospital_capacity as f32 {
        0.2
    } else { 0.0 }
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
    pub infection_rate: f32,
    pub hospitalization_rate: f32,
    pub max_hospital_capacity: usize,
    pub measures: Vec<Box<MeasureFn>>
}