pub fn float_power(value: f32, power: u32) -> f32 {
    let mut res: f32 = 1.0;
    for _ in 0..power {
        res *= value;
    }
    return res;
}

pub fn square_root(value: f32) -> f32 {
    let precision: f32 = 0.001;
    let mut res: f32 = value;
    while (res - value / res) > precision {
        res = (res + value / res) / 2.0;
    }
    return res;
}
