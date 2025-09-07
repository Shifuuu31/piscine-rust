pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value: f64 = (c as f64).exp();
    let ln_value: f64 = (c as f64).abs().ln();

    (c, exp_value, ln_value)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_str = a
        .split(' ')
        .map(|n| n.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (a, exp_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_vec: Vec<f64> = b
        .iter()
        .map(|&n| (n as f64).abs().ln())
        .collect::<Vec<f64>>();
    (b, ln_vec)
}
