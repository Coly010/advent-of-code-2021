use std::collections::HashMap;

// const INPUT: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize
}

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let coords = get_lines_from_input();

    let mut coord_map: HashMap<Coord, usize> = HashMap::new();

    for pair in coords {
        if pair[0].x != pair[1].x && pair[0].y != pair[1].y {
            continue;
        }

        if pair[0].x == pair[1].x {
            let range = if pair[0].y < pair[1].y { pair[0].y..=pair[1].y } else { pair[1].y..=pair[0].y };
            for i in range {
                let count = coord_map.entry(Coord {x: pair[0].x, y: i}).or_insert(0);
                *count += 1;
            }
        } else if pair[0].y == pair[1].y {
            let range = if pair[0].x < pair[1].x { pair[0].x..=pair[1].x } else { pair[1].x..=pair[0].x };
            for i in range {
                let count = coord_map.entry(Coord {x: i, y: pair[0].y}).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut count = 0;
    for val in coord_map.values() {
        count = if val >= &2 { count+1 } else {count}
    }

    println!("Problem 1: {}", count);

}

fn problem2() {
    let coords = get_lines_from_input();

    let mut coord_map: HashMap<Coord, usize> = HashMap::new();

    for pair in coords {
        if pair[0].x == pair[1].x {
            let range = if pair[0].y < pair[1].y { pair[0].y..=pair[1].y } else { pair[1].y..=pair[0].y };
            for i in range {
                let count = coord_map.entry(Coord {x: pair[0].x, y: i}).or_insert(0);
                *count += 1;
            }
        } else if pair[0].y == pair[1].y {
            let range = if pair[0].x < pair[1].x { pair[0].x..=pair[1].x } else { pair[1].x..=pair[0].x };
            for i in range {
                let count = coord_map.entry(Coord {x: i, y: pair[0].y}).or_insert(0);
                *count += 1;
            }
        } else {
            let (mut start_x, mut start_y) = if pair[0].y > pair[1].y { (pair[1].x, pair[1].y) } else { (pair[0].x, pair[0]. y) };
            let (mut end_x, mut end_y) = if pair[0].y > pair[1].y { (pair[0].x, pair[0].y) } else { (pair[1].x, pair[1].y) };

            while start_y <= end_y {
                let count = coord_map.entry(Coord {x: start_x, y: start_y}).or_insert(0);
                *count += 1;

                if start_x > end_x {
                    start_x -= 1;
                } else if start_x < end_x {
                    start_x += 1;
                } 

                start_y += 1;
            }
        }
    }

    let mut count = 0;
    for val in coord_map.values() {
        count = if val >= &2 { count+1 } else {count}
    }

    println!("Problem 2: {}", count);
}

fn get_lines_from_input() -> Vec<Vec<Coord>> {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let mut coord_pairs: Vec<Vec<Coord>> = Vec::new();

    for line in lines {
        let coords = line.split("->")
            .collect::<Vec<&str>>();
        
        let mut coord_pair: Vec<Coord> = Vec::new();
        for coord in coords {

            let pair = coord.trim().split(",")
                .collect::<Vec<&str>>();
            
                coord_pair.push(Coord {
                    x: pair[0].parse::<usize>().unwrap(),
                    y: pair[1].parse::<usize>().unwrap()
                });
        }
        coord_pairs.push(coord_pair);     
    }

    coord_pairs
}