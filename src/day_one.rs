pub fn part_one(lines: Vec<String>) -> i64 {
    let initial: f64 = lines.first().unwrap().parse().unwrap();
    return get_mass(initial);
}

fn part_two() {}

fn get_mass(mass: f64) -> i64 {
    return (mass / 3f64).floor() as i64 - 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mass() {
        assert_eq!(get_mass(12f64), 2);
        assert_eq!(get_mass(14f64), 2);
        assert_eq!(get_mass(1969f64), 654);
        assert_eq!(get_mass(100756f64), 33583);
    }
}
