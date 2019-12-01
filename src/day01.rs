use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod part01 {
    // Fuel required to launch a given module is based on its mass. Specifically,
    // to find the fuel required for a module, take its mass, divide by three,
    // round down, and subtract 2.
    pub fn calculate_fuel(mass: f32) -> f32 {
        (mass / 3.0).floor() - 2.0
    }

    pub fn solve(input: &[f32]) -> f32 {
        input.iter().cloned().map(calculate_fuel).sum()
    }
}

mod part02 {
    // Fuel itself requires fuel just like a module - take its mass, divide by
    // three, round down, and subtract 2. However, that fuel also requires fuel,
    // and that fuel requires fuel, and so on. Any mass that would require
    // negative fuel should instead be treated as if it requires zero fuel; the
    // remaining mass, if any, is instead handled by wishing really hard, which
    // has no mass and is outside the scope of this calculation.
    //
    // So, for each module mass, calculate its fuel and add it to the total.
    // Then, treat the fuel amount you just calculated as the input mass and
    // repeat the process, continuing until a fuel requirement is zero or
    // negative.
    pub fn calculate_fuel(module_mass: f32) -> f32 {
        let mut total_mass = 0.0;
        let mut fuel_mass = super::part01::calculate_fuel(module_mass);
        while fuel_mass > 0.0 {
            total_mass += fuel_mass;
            fuel_mass = super::part01::calculate_fuel(fuel_mass);
        }
        total_mass
    }

    pub fn solve(input: &[f32]) -> f32 {
        input.iter().cloned().map(calculate_fuel).sum()
    }
}

fn get_input<T: std::str::FromStr>() -> Result<Vec<T>, Box<dyn Error>> {
    let f = File::open("../inputs/day01.txt")?;
    let reader = BufReader::new(f);
    let fuel: Vec<T> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse().ok())
        .collect();
    Ok(fuel)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let fuel = part01::solve(&input[..]);
    println!("{}", file!());
    println!("part 1: total fuel required for modules: {}", fuel);
    let fuel = part02::solve(&input[..]);
    println!("part 2: total fuel required for modules + fuel: {}", fuel);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel1() {
        let cases: &[(f32, f32)] = &[
            (12.0, 2.0),
            (14.0, 2.0),
            (1969.0, 654.0),
            (100756.0, 33583.0),
        ];
        for (mass, fuel) in cases {
            assert_eq!(part01::calculate_fuel(*mass), *fuel);
        }
    }

    #[test]
    fn test_fuel2() {
        let cases: &[(f32, f32)] = &[(14.0, 2.0), (1969.0, 966.0), (100756.0, 50346.0)];
        for (mass, fuel) in cases {
            assert_eq!(part02::calculate_fuel(*mass), *fuel);
        }
    }
}
