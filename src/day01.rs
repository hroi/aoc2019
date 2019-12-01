use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod part01 {
    // Fuel required to launch a given module is based on its mass. Specifically,
    // to find the fuel required for a module, take its mass, divide by three,
    // round down, and subtract 2.
    pub fn calculate_fuel(mass: i32) -> i32 {
        (mass / 3) - 2
    }

    pub fn solve(input: &[i32]) -> i32 {
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
    pub fn calculate_fuel(mass: i32) -> i32 {
        let mut total_fuel = 0;
        let mut fuel = super::part01::calculate_fuel(mass);
        while fuel > 0 {
            total_fuel += fuel;
            fuel = super::part01::calculate_fuel(fuel);
        }
        total_fuel
    }

    pub fn solve(input: &[i32]) -> i32 {
        input.iter().cloned().map(calculate_fuel).sum()
    }
}

fn get_input<T: std::str::FromStr>() -> Result<Vec<T>, Box<dyn Error>> {
    let f = File::open("inputs/day01.txt")?;
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
        let cases: &[(i32, i32)] = &[(12, 2), (14, 2), (1969, 654), (100756, 33583)];
        for (mass, fuel) in cases {
            assert_eq!(part01::calculate_fuel(*mass), *fuel);
        }
    }

    #[test]
    fn test_fuel2() {
        let cases: &[(i32, i32)] = &[(14, 2), (1969, 966), (100756, 50346)];
        for (mass, fuel) in cases {
            assert_eq!(part02::calculate_fuel(*mass), *fuel);
        }
    }
}
