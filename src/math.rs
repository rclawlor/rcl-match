/// Round value to n decimal places
pub fn round(value: f32, n: usize) -> f32 {
    let scale: f32 = 10.0_f32.powf(n as f32);
    (value * scale).round() / scale
}
