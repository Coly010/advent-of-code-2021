use std::ptr::null;

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let positions = get_positions();
    println!(
        "Problem 1: {}",
        get_position_alignment_with_least_fuel(&positions)
    );
}

fn problem2() {
    let positions = get_positions();
    println!(
        "Problem 2: {}",
        get_position_alignment_with_least_fuel_p2(&positions)
    );
}

fn get_positions() -> Vec<usize> {
    return INPUT
        .split(",")
        .flat_map(|pos| pos.parse::<usize>().ok())
        .collect::<Vec<usize>>();
}

fn get_position_alignment_with_least_fuel(positions: &Vec<usize>) -> usize {
    let mut least_fuel_pos: Option<usize> = None;
    let mut least_fuel_needed: Option<usize> = None;
    for pos in positions {
        if least_fuel_pos == Some(*pos) {
            continue;
        }
        let fuel_needed = get_sum_of_fuel_to_move_to_position(positions, pos);

        if least_fuel_needed == None || least_fuel_needed > Some(fuel_needed) {
            least_fuel_needed = Some(fuel_needed);
            least_fuel_pos = Some(*pos);
        }
    }
    least_fuel_needed.unwrap()
}

fn get_sum_of_fuel_to_move_to_position(positions: &Vec<usize>, position: &usize) -> usize {
    let mut total_fuel = 0;
    for pos in positions {
        total_fuel += if pos > position {
            pos - position
        } else {
            position - pos
        };
    }

    total_fuel
}

fn get_position_alignment_with_least_fuel_p2(positions: &Vec<usize>) -> usize {
    let mut least_fuel_pos: Option<usize> = None;
    let mut least_fuel_needed: Option<usize> = None;
    for pos in positions {
        if least_fuel_pos == Some(*pos) {
            continue;
        }
        let fuel_needed = get_sum_of_fuel_to_move_to_position_p2(
            positions,
            pos,
            least_fuel_needed.unwrap_or(10000000000),
        );

        if least_fuel_needed == None || least_fuel_needed > Some(fuel_needed) {
            least_fuel_needed = Some(fuel_needed);
            least_fuel_pos = Some(*pos);
        }
    }
    least_fuel_needed.unwrap()
}

fn get_sum_of_fuel_to_move_to_position_p2(
    positions: &Vec<usize>,
    position: &usize,
    current_least: usize,
) -> usize {
    let mut total_fuel = 0;
    for pos in positions {
        let positions_to_move = if pos > position {
            pos - position
        } else {
            position - pos
        };
        let new_fuel = positions_to_move * (positions_to_move + 1) / 2;
        total_fuel += new_fuel
    }

    total_fuel
}
