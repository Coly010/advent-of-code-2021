use std::collections::BTreeMap;

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

fn main() {
    problem1();
    println!();
    problem2();
}

fn problem1() {
    let population = get_initial_population();
    let mut population_map = create_btree_map_of_ages(&population);
    for i in 0..80 {
        propogate_day(&mut population_map);
    }

    println!("Problem 1: {}", count_population(&population_map))
}

fn problem2() {
    let population = get_initial_population();
    let mut population_map = create_btree_map_of_ages(&population);
    for i in 0..256 {
        propogate_day(&mut population_map);
    }

    println!("Problem 2: {}", count_population(&population_map))
}

fn get_initial_population() -> Vec<usize> {
    let unparsed_population = INPUT.split(",");

    let mut parsed_population: Vec<usize> = Vec::new();

    for unparsed_fish in unparsed_population {
        let fish = unparsed_fish.parse::<usize>();
        match fish {
            Ok(parsed_fish) => parsed_population.push(parsed_fish),
            Err(error) => println!("Failed to parse fish - {:?}", error),
        }
    }

    parsed_population
}

fn create_btree_map_of_ages(population: &Vec<usize>) -> BTreeMap<usize, usize> {
    let mut population_map = BTreeMap::new();
    for fish in population {
        let count = population_map.entry(*fish).or_insert(0);
        *count += 1;
    }
    population_map
}

fn propogate_day(population: &mut BTreeMap<usize, usize>) {
    let mut spawned_fish = 0;
    for (age, count) in population.clone().iter() {
        if *age == 0 {
            spawned_fish += *count;
        } else {
            population.insert(*age, 0);
            let old_count = population.entry(*age - 1).or_insert(0);
            *old_count = *count;
        }
    }

    let count = population.entry(6).or_insert(0);
    *count += spawned_fish;

    let count = population.entry(8).or_insert(0);
    *count += spawned_fish;
}

fn count_population(population: &BTreeMap<usize, usize>) -> usize {
    population
        .values()
        .cloned()
        .into_iter()
        .fold(0, |population, fish| population + fish)
}
