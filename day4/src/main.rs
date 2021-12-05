// const INPUT: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

struct BoardValue {
    value: String,
    marked: bool
}

struct Board {
    id: usize,
    values: Vec<BoardValue>,
}

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let called_numbers = lines[0].split(",").collect::<Vec<&str>>();

    let board_lines = &lines[2..];
    let mut raw_board_values: Vec<Vec<&str>> = Vec::new();

    let mut current_board: Vec<&str> = Vec::new();
    for line in board_lines {
        if line.chars().nth(0) == None {
            raw_board_values.push(current_board);
            current_board = Vec::new();
            continue;
        }

        current_board.push(line);
    }
    raw_board_values.push(current_board);

    let mut boards: Vec<Board> = Vec::new();
    for i in 0..raw_board_values.len() {
        boards.push(build_board(&raw_board_values[i], i));
    }

    'outer: for number in called_numbers {
        call_number(number, &mut boards);
        for board in boards.iter() {
            let board_won = check_board_for_win(board);
            if board_won == true {
                let score = calculate_board_score(board, number);

                println!("Problem 1: {}", score);
                break 'outer;
            }
        }
    }

}

fn problem2() {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let called_numbers = lines[0].split(",").collect::<Vec<&str>>();

    let board_lines = &lines[2..];
    let mut raw_board_values: Vec<Vec<&str>> = Vec::new();

    let mut current_board: Vec<&str> = Vec::new();
    for line in board_lines {
        if line.chars().nth(0) == None {
            raw_board_values.push(current_board);
            current_board = Vec::new();
            continue;
        }

        current_board.push(line);
    }
    raw_board_values.push(current_board);

    let mut boards: Vec<Board> = Vec::new();
    for i in 0..raw_board_values.len() {
        boards.push(build_board(&raw_board_values[i], i));
    }

    let mut boards_won: Vec<usize> = Vec::new();

    'outer: for number in called_numbers {
        call_number(number, &mut boards);
        for board in boards.iter() {
            let board_won = check_board_for_win(board);
            if board_won == true {
                if boards_won.contains(&board.id) == false {
                    boards_won.push(board.id);
                }
    
                if boards_won.len() == boards.len() {
                    let score = calculate_board_score(board, number);

                    println!("Problem 2: {}", score);
                    break 'outer;
                }
            }
        }
    }
}

fn build_board(raw_board: &Vec<&str>, id: usize) -> Board {
    let mut values: Vec<BoardValue> = Vec::new();
    for line in raw_board {
        line.split_whitespace().collect::<Vec<&str>>().iter().map(|&val| 
            BoardValue {
                value: String::from(val),
                marked: false
            }
        ).for_each(|val| { 
            values.push(val); 
        });
    }

    Board { id, values }
}

fn get_rows_columns_from_board(board: &Board) -> (Vec<Vec<&BoardValue>>, Vec<Vec<&BoardValue>>) {
    let mut rows: Vec<Vec<&BoardValue>> = Vec::new();
    let mut current_row: Vec<&BoardValue> = Vec::new();
    for i in 0..board.values.len() {
        if i != 0 && i % 5 == 0 {
            rows.push(current_row);
            current_row = Vec::new();
        }

        current_row.push(&board.values[i]);
    }
    rows.push(current_row);

    let mut columns: Vec<Vec<&BoardValue>> = Vec::new();
    for i in 0..rows.len() {
        let mut column: Vec<&BoardValue> = Vec::new();
        for row in &rows {
            column.push(&row[i]);
        }
        columns.push(column);
    }
    
    (rows, columns)
}

fn call_number(number: &str, boards: &mut Vec<Board>) {
    for board in boards.iter_mut() {
        for val in board.values.iter_mut() {
            if val.value == number {
                val.marked = true;
            }
        }
    }
}

fn check_board_for_win(board: &Board) -> bool {
    let (rows, columns) = get_rows_columns_from_board(board);

    let mut win = false;
    for row in rows {
        let mut row_win = true;
        for val in row {
            if !val.marked {
                row_win = false;
            }
        }

        if row_win == true {
            win = true;
        }
    }

    if win == true {
        true;
    }

    for column in columns {
        let mut column_win = true;
        for val in column {
            if !val.marked {
                column_win = false;
            }
        }

        if column_win == true {
            win = true;
        }
    }

    win
}

fn calculate_board_score(board: &Board, final_number: &str) -> i32 {
    let mut sum_of_unmarked = 0;
    for val in board.values.iter() {
        if val.marked == true {
            continue;
        }
        
        sum_of_unmarked = sum_of_unmarked + val.value.parse::<i32>().unwrap();
    }

    sum_of_unmarked * final_number.parse::<i32>().unwrap()
}