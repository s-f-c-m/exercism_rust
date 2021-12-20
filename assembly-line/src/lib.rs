// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    let brute_production: u64 = speed as u64 * 221;
    if speed < 5 {
        return brute_production as f64;
    }
    if speed < 9 {
        return (brute_production as f64) * 0.9;
    }
    (brute_production as f64) * 0.77
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    (production_rate_per_hour(speed) / (60) as f64) as u32
}
