use std::{env, fs};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
struct Coordinate {
    row: u8,
    column: u8,
    number: u8,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut difficulty: u8 = 1;  // Difficulty of the generated sudoku [1..5]
    let mut amount: u8 = 1;      // Amount of generated sudokus [1..10]
    let mut solved = true;  // If the generated sudoku should include a solved version [true|false]
    let mut json = false;   // If the generated sudoku should be also exported in JSON format [true|false]

    for (index, argument) in args.iter().enumerate().skip(1) {
        match argument.as_str() {
            "-h" | "--help" => {
                println!("\n\x1b[1m./sudoku-generator \x1b[0m\x1b[33m[-option]\x1b[0m\x1b[1m <argument>\x1b[0m    \x1b[2m(Can be empty and repeated)\x1b[0m\n");
                println!("\x1b[33moptions\x1b[0m:\n  \x1b[33m-h\x1b[0m -> shows infos about this app\n  \x1b[33m-d\x1b[0m -> changes difficulty [1..10] std: 1 (./sudoku-generator -d 3)\n  \x1b[33m-s\x1b[0m -> show only solved sudoku (./sudoku-generator -s)\n  \x1b[33m-a\x1b[0m -> changes the amount of generated Sudokus [1..10] std: 1 (./sudoku-generator -a 3) \x1b[2m((Creates <amount> times of files))\x1b[0m\n  \x1b[33m-j\x1b[0m -> creates an extra json file with information about the sudoku\n");

                std::process::exit(0);
            }
            "-d" => {
                if args.len() <= index+1{
                    eprintln!("\x1b[31mmissing argument\x1b[0m");
                    std::process::exit(1);
                }else if ["1","2","3","4","5","6","7","8","9","10","15"].contains(&&*args[index + 1]){
                    difficulty = (&&*args[index + 1]).parse::<u8>().unwrap()
                }else{
                    eprintln!("Wrong argument");
                    std::process::exit(1);
                }
            }
            "-s" => {
                solved = false;
            }
            "-a" => {
                if args.len() <= index+1{
                    eprintln!("\x1b[31mmissing argument\x1b[0m");
                    std::process::exit(1);
                }else if ["1","2","3","4","5","6","7","8","9","10"].contains(&&*args[index + 1]){
                    amount = (&&*args[index + 1]).parse::<u8>().unwrap()
                }else{
                    eprintln!("Wrong argument");
                    std::process::exit(1);
                }
            }
            "-j" => {
                json = true;
            }
            _ => {}
        }
    }

    for _ in 0..amount {
        let mut field: Vec<Coordinate> = vec![];

        for i in 1..=9 {
            for k in 1..=9 {
                field.push(Coordinate {
                    row: i,
                    column: k,
                    number: 0,
                });
            }
        }

        if fill_sudoku(&mut field) {
            write_sudoku_to_file(&field, difficulty, solved, json);
        } else {
            println!("Failed to generate a complete Sudoku.");
        }
    }
}

fn fill_sudoku(field: &mut Vec<Coordinate>) -> bool {
    let mut rng = rand::thread_rng();

    let empty_position = field.iter().position(|coord| coord.number == 0);

    if let Some(idx) = empty_position {
        let row_start = (field[idx].row as usize - 1) * 9;
        let column_start = field[idx].column as usize - 1;

        let block_row_start = (field[idx].row as usize - 1) / 3 * 3;
        let block_column_start = (field[idx].column as usize - 1) / 3 * 3;

        let mut numbers: Vec<u8> = (1..=9).collect();
        numbers.shuffle(&mut rng);

        for &num in &numbers {
            let row_conflict = (0..9).any(|i| field[row_start + i].number == num);
            let column_conflict = (0..9).any(|i| field[column_start + i * 9].number == num);
            let block_conflict = (0..3).any(|i| {
                (0..3).any(|j| {
                    let block_index = (block_row_start + i) * 9 + (block_column_start + j);
                    field[block_index].number == num
                })
            });

            if !row_conflict && !column_conflict && !block_conflict {
                field[idx].number = num;

                if fill_sudoku(field) {
                    return true;
                }

                field[idx].number = 0;
            }
        }

        return false;
    }

    true
}

fn write_sudoku_to_file(field: &Vec<Coordinate>, difficulty: u8, solved: bool, json: bool) {
    let mut rng = rand::thread_rng();

    let mut file_name = "sudokus/sudoku".to_string();
    for _ in 0..9 {
        let added_num: u8 = rng.gen_range(1..=9);
        file_name += format!("{}", added_num).as_str();
    }
    file_name += format!("-{}", difficulty).as_str();
    let mut json_file_name = file_name.clone();
    file_name += ".txt";

    fs::create_dir_all("sudokus").expect("Unable to create directory");
    let mut file = File::create(file_name).expect("Unable to create file");

    if solved {
        for coord in field {
            write!(file, " {}", coord.number).expect("Unable to write to file");
            if coord.column % 3 == 0 && coord.column != 9 && coord.column != 0 {
                write!(file, " |").expect("Unable to write to file");
            }
            if coord.column == 9 {
                writeln!(file).expect("Unable to write to file");
                if coord.row % 3 == 0 && coord.row != 0 && coord.row != 9 {
                    writeln!(file, " ------|-------|------").expect("Unable to write to file");
                }
            }
        }
        writeln!(file).expect("Unable to write to file");
        writeln!(file).expect("Unable to write to file");
    }

    for coord in field {
        let show = rng.gen_range(0..difficulty + 1);
        if show == 1 {
            write!(file, " {}", coord.number).expect("Unable to write to file");
        } else {
            write!(file, "  ").expect("Unable to write to file");
        }
        if coord.column % 3 == 0 && coord.column != 9 && coord.column != 0 {
            write!(file, " |").expect("Unable to write to file");
        }
        if coord.column == 9 {
            writeln!(file).expect("Unable to write to file");
            if coord.row % 3 == 0 && coord.row != 0 && coord.row != 9 {
                writeln!(file, " ------|-------|------").expect("Unable to write to file");
            }
        }
    }

    if json {
        json_file_name += ".json";
        let num = &json_file_name.clone()[14..23];
        let mut json_file = File::create(json_file_name).expect("Unable to create file");

        writeln!(json_file, "{}\n  \"difficulty\": {},\n  \"id\": {},\n  \"field\": {}","{",difficulty,num,"{").expect("Unable to write to file");
        for coord in field {
            if coord.column == 1 {
                write!(json_file,"    \"row{}\": [",coord.row).expect("Unable to write to file");
            }
            if coord.column != 9 {
                write!(json_file, "{},", coord.number).expect("Unable to write to file");
            } else {
                write!(json_file, "{}", coord.number).expect("Unable to write to file");
            }
            if coord.column == 9 {
                write!(json_file,"]").expect("Unable to write to file");
                if coord.row != 9 {
                    writeln!(json_file,",").expect("Unable to write to file");
                }
            }
        }
        writeln!(json_file, "{}","\n  }\n}").expect("Unable to write to file");
    }
}
