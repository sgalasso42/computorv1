use crate::utils;
use crate::parsing;

use parsing::parser::{Monomial};
use utils::math::{float_power};

pub fn reduce(monomial_list: &Vec<Monomial>) -> Result<[f32; 3], String> {
    let mut polynomial: [f32; 3] = [0.0; 3];
    let max_degree: u32 = monomial_list.iter().max_by(|x, y| x.power.cmp(&y.power)).unwrap().power;
    for degree in 0..(max_degree + 1) {
        let sum: f32 = monomial_list.iter().filter(|monomial| monomial.power == degree).map(|el| el.factor).sum();
        if degree < 3 {
            polynomial[degree as usize] = sum;
        } else if sum != 0.0 {
            return Err(String::from("Error, the polynomial degree is stricly greater than 2, I can't solve."));
        }
    }
    polynomial.reverse();
    return Ok(polynomial);
}

pub fn display_reduced(polynomial: &[f32; 3]) {
    let mut s: String = String::new();
    if polynomial[2] != 0.0 {
        s.push_str(&format!("{}", polynomial[2]));
    }
    match polynomial[1] {
        factor if polynomial[2] == 0.0 && factor > 0.0 => s.push_str(&format!("{} * X", factor)),
        factor if factor > 0.0 => s.push_str(&format!(" + {} * X", factor)),
        factor if factor < 0.0 => s.push_str(&format!(" - {} * X", -factor)),
        _ => {}
    }
    match polynomial[0] {
        factor if polynomial[2] == 0.0 && polynomial[1] == 0.0 && factor > 0.0 => s.push_str(&format!("{} * X^2", factor)),
        factor if factor > 0.0 => s.push_str(&format!(" + {} * X^2", factor)),
        factor if factor < 0.0 => s.push_str(&format!(" - {} * X^2", -factor)),
        _ => {}
    };
    println!("Reduced form: {} = 0", s);
}

pub fn get_discriminant(polynomial: &[f32; 3]) -> f32 {
    let discriminant: f32 = float_power(polynomial[1], 2) - 4.0 * polynomial[0] * polynomial[2];
    println!("Discriminant: b^2 - 4 * a * c = {}^2 - 4 * {} * {} = {}", polynomial[1], polynomial[0], polynomial[2], discriminant);
    return discriminant;
}