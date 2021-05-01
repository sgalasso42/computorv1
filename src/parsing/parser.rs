extern crate regex;

#[derive(Debug, PartialEq)]
pub struct Monomial {
    pub factor: f32,
    pub power: u32
}

pub fn parse(raw_input: &str) -> Result<Vec<Monomial>, String> {
    let input: &String = &raw_input.replace(" ", "");
    if input.matches('=').count() != 1 {
        return Err(String::from("Error, there should be exactly one = character in the expression"));
    }
    let mut monomial_list: Vec<Monomial> = vec![];
    let monomial_pattern = regex::Regex::new(r"=?[+-]?(\d+(\.\d+)?\*X\^\d+|\d+(\.\d+)?\*X|X\^\d+|\d+(\.\d+)?|X)").unwrap();
    let mut sign: f32 = 1.0;
    let mut cursor: usize = 0;
    for (i, mat) in monomial_pattern.find_iter(&input).enumerate() {
        if mat.start() != cursor {
            return Err(format!("Error, {} is invalid in the expression", &input[cursor..mat.start()]));
        } else {
            cursor = mat.end();
            let str_monomial: &str = &input[mat.start()..mat.end()];
            if i != 0 && !['+', '-', '*', '='].contains(&str_monomial.chars().nth(0).unwrap()) {
                return Err(format!("Error, the expression format has an issue at character '{}'", &str_monomial.chars().nth(0).unwrap()));
            }
            if str_monomial.chars().nth(0).unwrap() == '=' {
                sign = -1.0;
            }
            let cleaned_monomial: String = str_monomial.replace("+", "").replace("=", "").replace("-X", "-1*X");
            monomial_list.push(Monomial {
                factor: match cleaned_monomial.chars().position(|c| !['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(&c)) {
                    Some(index) => match index {
                        0 => 1.0 * sign,
                        i => match cleaned_monomial[..i].parse::<f32>() {
                            Ok(val) => match cleaned_monomial[..i].find('.').unwrap_or(i) <= std::f32::DIGITS as usize { 
                                true => sign * val,
                                false => { return Err("error, please use reasonable numbers ".to_string()); }
                            },
                            Err(err) => { return Err(err.to_string()); }
                        }
                    },
                    None => match cleaned_monomial.parse::<f32>() {
                        Ok(val) => match cleaned_monomial.find('.').unwrap_or(cleaned_monomial.len()) <= std::f32::DIGITS as usize {
                            true => sign * val,
                            false => { return Err("error, please use reasonable numbers ".to_string()); }
                        },
                        Err(err) => { return Err(err.to_string()); }
                    }
                },
                power: match cleaned_monomial.chars().position(|c| c == '^') {
                    Some(idx) => match cleaned_monomial[idx + 1..].parse::<u32>() {
                        Ok(val) => val,
                        Err(err) => { return Err(err.to_string()); }
                    }
                    None => match cleaned_monomial.contains("X") {
                        true => 1,
                        false => 0
                    }
                }
            })
        }
    }
    if cursor != input.len() {
        return Err(String::from("Error, bad format at the end of the expression"));
    }
    if monomial_list.iter().any(|el| el.factor.is_infinite()) {
        return Err(String::from("Error, bad format, please use reasonable numbers"));
    }
    return Ok(monomial_list);
}