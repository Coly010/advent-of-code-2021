use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    
    let instructions = contents.lines().collect::<Vec<&str>>();

    problem1(&instructions);
    problem2(&instructions);
}

fn problem1(instructions: &Vec<&str>) {
    let (mut hoz, mut depth) =  (0,0);

    for instruction in instructions {
        let split_instruction = instruction.split(' ').collect::<Vec<&str>>();
        let direction = split_instruction[0];
        let amount = split_instruction[1].parse::<i32>().unwrap();

        if direction == "forward" {
            hoz = hoz + amount;
        } else if direction == "down" {
            depth = depth + amount;
        } else if direction == "up" {
            depth = depth - amount;
        }
    }

    let result = hoz * depth;
    println!("Problem 1 Result: {}", result);
}

fn problem2(instructions: &Vec<&str>) {
    let (mut hoz, mut depth, mut aim) = (0,0,0);

    for instruction in instructions {
        let split_instruction = instruction.split(' ').collect::<Vec<&str>>();
        let direction = split_instruction[0];
        let amount = split_instruction[1].parse::<i32>().unwrap();

        if direction == "forward" {
            hoz = hoz + amount;
            depth = depth + (aim * amount);
        } else if direction == "down" {
            aim = aim + amount;
        } else if direction == "up" {
            aim = aim - amount;
        }
    }

    let result = hoz * depth;
    println!("Problem 2 Result: {}", result);
}