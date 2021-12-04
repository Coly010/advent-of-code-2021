// const INPUT: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let binary_numbers: Vec<&str> = INPUT.lines().collect();
    let transposed_matrix = transpose_and_parse_matrix(&binary_numbers);

    let number_of_columns = transposed_matrix.len();
    let mut gamma_rate: String = String::new();
    let mut epsilon_rate: String = String::new();

    for i in 0..number_of_columns {
        let mut number_of_zeros = 0;
        let mut number_of_ones = 0;

        for column in transposed_matrix[i].iter() {
            match column {
                0 => number_of_zeros = number_of_zeros + 1,
                1 => number_of_ones = number_of_ones + 1,
                _ => println!("Received unexpected value in matrix column")
            }
        }

        match (number_of_zeros, number_of_ones) {
            (zeros, ones) if zeros > ones => { 
                gamma_rate = gamma_rate + "0"; 
                epsilon_rate = epsilon_rate + "1";
            },
            (zeros, ones) if ones > zeros => {
                gamma_rate = gamma_rate + "1";
                epsilon_rate = epsilon_rate + "0";
            },
            _ => println!("Not sure what to do for equal results")
        }
    }

    let power_consumption = isize::from_str_radix(&gamma_rate, 2).unwrap() * isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Problem 1: {}", power_consumption)
}

fn problem2() {
    let mut current_matrix_of_numbers: Vec<&str> = INPUT.lines().collect();

    let length_of_string = current_matrix_of_numbers[0].len();

    for j in 0..length_of_string {
        if(current_matrix_of_numbers.len() == 1) {
            break;
        }

        let starting_value: char;
        let mut number_of_zeros = 0;
        let mut number_of_ones = 0;

        for i in 0..current_matrix_of_numbers.len() {
            match current_matrix_of_numbers[i].chars().nth(j).unwrap() {
                '0' => number_of_zeros = number_of_zeros + 1,
                '1' => number_of_ones = number_of_ones + 1,
                _ => println!("Encountered an unknown value")
            }
        }

        starting_value = if number_of_zeros > number_of_ones {'0'} else { '1'};  

        for i in (0..current_matrix_of_numbers.len()).rev() {
            if current_matrix_of_numbers[i].chars().nth(j).unwrap() != starting_value {
                current_matrix_of_numbers.remove(i);
            }
        }
    }

    let oxygen_rating = current_matrix_of_numbers[0];

    current_matrix_of_numbers = INPUT.lines().collect();

    for j in 0..length_of_string {
        if(current_matrix_of_numbers.len() == 1) {
            break;
        }

        let starting_value: char;
        let mut number_of_zeros = 0;
        let mut number_of_ones = 0;

        for i in 0..current_matrix_of_numbers.len() {
            match current_matrix_of_numbers[i].chars().nth(j).unwrap() {
                '0' => number_of_zeros = number_of_zeros + 1,
                '1' => number_of_ones = number_of_ones + 1,
                _ => println!("Encountered an unknown value")
            }
        }

        starting_value = if number_of_zeros <= number_of_ones {'0'} else { '1'};  

        for i in (0..current_matrix_of_numbers.len()).rev() {
            if current_matrix_of_numbers[i].chars().nth(j).unwrap() != starting_value {
                current_matrix_of_numbers.remove(i);
            }
        }
    }

    let co2_scrub_rating = current_matrix_of_numbers[0]; 

    let life_rating = isize::from_str_radix(oxygen_rating, 2).unwrap() * isize::from_str_radix(co2_scrub_rating, 2).unwrap();
    println!("Problem 2 {}", life_rating);
}

fn get_and_parse_column_as_vec(matrix: &Vec<&str>, column_index: usize) -> Vec<u32>{
    let mut column = Vec::<u32>::new();
    for row in matrix {
        let digit = row.chars().collect::<Vec<char>>()[column_index].to_digit(10).unwrap();
        column.push(digit);
    }

    return column;
}

fn transpose_and_parse_matrix(matrix: &Vec<&str>) -> Vec<Vec<u32>> {
    let mut columns: Vec<Vec<u32>> = Vec::new();
    
    let max_column_index = matrix[0].len();
    for i in 0..max_column_index {
        columns.push(get_and_parse_column_as_vec(&matrix, i));
    }

    return columns;
}

