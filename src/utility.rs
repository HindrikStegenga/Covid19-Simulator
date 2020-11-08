use serde::{Serialize, de::DeserializeOwned};
use std::io::Read;

// Macro which expands into color definition
#[macro_export]
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

// Loads file as binary object file.
pub fn load_file_bin(path: &str) -> Option<Vec<u8>> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;
    Some(bytes)
}

// Function which loads a JSON file and attempts to decode it into T.
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

// Generate float range from start to end using step as stepsize
pub fn generate_range(start: f32, end: f32, step: f32) -> Vec<f32> {
    assert!(end > start, "End must be larger than start");
    assert!(step > 0.0, "Step must be larger than 0.0");
    let steps = f32::ceil((end - start) / step) as usize;
    let mut vec = Vec::with_capacity(steps);
    for i in 0..steps+1 {
        vec.push(start + (i as f32) * step)
    }
    vec
}

// Generate float range based on certain length. Starts at 0
pub fn generate_range_from_input(input_len: usize, step: f32) -> Vec<f32> {
    let mut vec = Vec::with_capacity(input_len);
    for i in 0..input_len {
        vec.push(0.0 + (i as f32) * step)
    }
    vec
}