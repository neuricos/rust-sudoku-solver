use std::fs;
use std::process::exit;
use std::collections::HashMap;
use std::env;

fn get_board(fname: &str) -> [[u8; 9]; 9] {
    let contents: String = match fs::read_to_string(fname) {
        Ok(cnt) => { cnt },
        Err(_) => {
            eprintln!("Cannot open {}!", fname);
            exit(1);
        }
    };
    let lines: Vec<&str> = contents.lines().collect();
    if lines.len() != 9 {
        eprintln!("File {} is in bad format!", fname);
        eprintln!("(Correct format should have 9 lines)");
        exit(1);
    }

    let mut board: [[u8; 9]; 9] = [[0; 9]; 9];

    for (i, line) in lines.iter().enumerate() {
        let nums: Vec<&str> = line.split(",").collect();
        if nums.len() != 9 {
            eprintln!("File {} is in bad format at line {}!", fname, i+1);
            eprintln!("(Correct format should have 9 digits separated by comma)");
            exit(1);
        }
        for (j, num) in nums.iter().enumerate() {
            let digit: u8 = match num.to_string().parse::<u8>() {
                Ok(d) => { d },
                Err(_) => {
                    eprintln!("File {} is in bad format at line {}!", fname, i+1);
                    eprintln!("(Illegal digit {} found in line)", num);
                    exit(1);
                }
            };
            if digit > 9 {
                eprintln!("Illegal digit {} found in file {} at line {}!", num, fname, i+1);
                eprintln!("(Legitimate digit should be ranged from 0 to 9, inclusive)");
                exit(1);
            }
            // Everything is ok
            // Update the board
            board[i][j] = digit;
        }
    }

    board
}

fn print_board(board: &[[u8; 9]; 9]) {
    for i in 0..9 {
        print!("[");
        for j in 0..9 {
            if j != 0 {
                print!(",");
            }
            print!(" {}", board[i][j]);
        }
        println!(" ]");
    }
}

fn possible(board: &[[u8; 9]; 9], cell: (usize, usize), value: u8) -> bool {
    let (cr, cc) = cell;

    for c in 0..9 {
        if board[cr][c] == value {
            return false;
        }
    }

    for r in 0..9 {
        if board[r][cc] == value {
            return false;
        }
    }

    let rs = cr / 3 * 3;
    let cs = cc / 3 * 3;

    for i in rs..rs+3 {
        for j in cs..cs+3 {
            if board[i][j] == value {
                return false;
            }
        }
    }

    true
}

fn get_all_possible_values(board: &mut [[u8; 9]; 9]) -> HashMap<(usize, usize), Vec<u8>> {
    let mut all_possible_values = HashMap::<(usize, usize), Vec<u8>>::new();

    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == 0 {
                let possible_values: Vec<u8> = (1..10).filter(|v| possible(board, (r, c), *v)).collect();

                if possible_values.len() == 1 {
                    board[r][c] = possible_values[0];
                    return get_all_possible_values(board);
                }

                let to_explore = if possible_values.len() == 0 { false } else { true };

                all_possible_values.insert((r, c), possible_values);

                if !to_explore { break; }
            }
        }
    }

    all_possible_values
}

fn complete(board: &[[u8; 9]; 9]) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if !(1..10).contains(&board[i][j]) {
                return false;
            }
        }
    }
    true
}

fn solve(board: &mut [[u8; 9]; 9]) {
    let mut board_stack: Vec<[[u8; 9]; 9]> = Vec::<[[u8; 9]; 9]>::new();

    let all_possible_values = get_all_possible_values(board);

    if all_possible_values.len() == 0 {
        if complete(board) {
            println!("solution:");
            print_board(board);
            return;
        }
        println!("No solution possible!");
        exit(0);
    }

    let mut target_cell: &(usize, usize) = all_possible_values.keys().next().unwrap();
    let mut target_possible_values: &Vec<u8> = all_possible_values.get(target_cell).unwrap();

    for cell in all_possible_values.keys().skip(1) {
        let possible_values = all_possible_values.get(cell).unwrap();
        if possible_values.len() == 0 {
            println!("No solution possible!");
            exit(0);
        }

        if target_possible_values.len() > possible_values.len() {
            target_cell = cell;
            target_possible_values = possible_values;
        }
    }

    let &(cr, cc) = target_cell;

    for tpv in target_possible_values.iter() {
        let mut new_board = board.clone();
        new_board[cr][cc] = *tpv;
        board_stack.push(new_board);
    }

    while board_stack.len() != 0 {
        let mut next_board = board_stack.pop().unwrap();
        let all_possible_values = get_all_possible_values(&mut next_board);

        if all_possible_values.len() == 0 {
            println!("solution:");
            print_board(&next_board);
            continue;
        }

        let mut target_cell: &(usize, usize) = all_possible_values.keys().next().unwrap();
        let mut target_possible_values: &Vec<u8> = all_possible_values.get(target_cell).unwrap();
        let mut solution_impossible = false;

        for cell in all_possible_values.keys().skip(1) {
            let possible_values = all_possible_values.get(cell).unwrap();
            if possible_values.len() == 0 {
                solution_impossible = true;
                break;
            }

            if target_possible_values.len() > possible_values.len() {
                target_cell = cell;
                target_possible_values = possible_values;
            }
        }

        if solution_impossible {
            continue;
        }

        let &(cr, cc) = target_cell;
        for tpv in target_possible_values.iter() {
            let mut new_board = board.clone();
            new_board[cr][cc] = *tpv;
            board_stack.push(new_board);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname: &str = &args[1];
    let mut board = get_board(fname);
    solve(&mut board);
}