mod parsing;
mod utils;

use fraction::Fraction;
use parsing::args::{Config};
use parsing::parser::{parse};
use utils::math::{square_root};
use utils::polynomial::*;

fn main() {
    let config: Config = Config::new();
    match parse(&config.equation) {
        Ok(monomial_list) => {
            match reduce(&monomial_list) {
                Ok(polynomial) => {
                    println!("abc: {:?}", polynomial);
                    // the line bellow is used to find the index of the first non-null value and reverse it to have the order : 0=a 1=b 2=c
                    let degree: usize = (2 - polynomial.iter().position(|factor| *factor != 0.0).unwrap_or(2) as i32).abs() as usize; 
                    println!("Polynomial degree: {}", degree);
                    match degree {
                        0 => match config.equation.contains("X") {
                            true => match polynomial.iter().sum::<f32>() == 0.0 {
                                true => println!("Every real numbers are solutions"),
                                false => println!("There is no solutions")
                            }
                            false => println!("The equation is {}", polynomial.iter().sum::<f32>() == 0.0)
                        },
                        1 => {
                            display_reduced(&polynomial);
                            println!("The unique solution is:");
                            let res: f32 = -polynomial[2] / polynomial[1];
                            let fract: Fraction = Fraction::from(-polynomial[2]) / Fraction::from(polynomial[1]);
                            println!("-c / b = {} / {} = {} = {:?}", -polynomial[2], polynomial[1], fract, res);
                        },
                        2 => {
                            display_reduced(&polynomial);
                            match get_discriminant(&polynomial) {
                                d if d > 0.0 => {
                                    println!("Discriminant is strictly positive, the two solutions are:");
                                    let res0: f32 = (-polynomial[1] + square_root(d)) / (2.0 * polynomial[0]);
                                    let fract0: Fraction = Fraction::from(-polynomial[1] + square_root(d)) / Fraction::from(2.0 * polynomial[0]);
                                    println!("(-b + √(delta)) / (2 * a) = ({} + √({})) / (2 * {}) = {} = {}", -polynomial[1], d, polynomial[0], fract0, res0);
                                    let res1: f32 = (-polynomial[1] - square_root(d)) / (2.0 * polynomial[0]);
                                    let fract1: Fraction = Fraction::from(-polynomial[1] - square_root(d)) / Fraction::from(2.0 * polynomial[0]);
                                    println!("(-b - √(delta)) / (2 * a) = ({} - √({})) / (2 * {}) = {} = {}", -polynomial[1], d, polynomial[0], fract1, res1);
                                },
                                d if d < 0.0 => {
                                    println!("Discriminant is strictly negative, the two solutions are:");
                                    println!("(-b + i * √(-delta)) / (2 * a) = ({} + i * √({})) / {}", -polynomial[1], -d, 2.0 * polynomial[0]);
                                    println!("(-b - i * √(-delta)) / (2 * a) = ({} - i * √({})) / {}", -polynomial[1], -d, 2.0 * polynomial[0]);
                                },
                                _ => {
                                    println!("Discriminant equal zero, the solution is:");
                                    let res: f32 = -polynomial[1] / (2.0 * polynomial[0]);
                                    let fract: Fraction = Fraction::from(-polynomial[1]) / Fraction::from(2.0 * polynomial[0]);
                                    println!("-b / (2 * a) = {} / (2 * {}) = {} = {}", -polynomial[1], polynomial[0], fract, res);
                                }
                            }
                        }
                        _ => panic!("Impossible case")
                    }
                },
                Err(error) => println!("{}", error)
            }
        },
        Err(error) => println!("{}", error)
    }
}
