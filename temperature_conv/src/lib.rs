// °C = (°F - 32) ÷ (9/5)
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.) / (9. / 5.)
}

// °F = °C * 9/5 + 32
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9. / 5.) + 32.
}