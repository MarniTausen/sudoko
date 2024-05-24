use colored::Colorize;
use sudoko::model::{Sudoko, SudokoValue};
use sudoko::cursor::{self, Cursor};
use console::Term;
use std::{thread, time::Duration};

fn main() {
    let mut sudoko = Sudoko::new();
    let stdout = Term::buffered_stdout();
    let mut cursor = Cursor::new();

    /*sudoko.set_value(1, 1, SudokoValue::One(false)).unwrap();
    sudoko.set_value(1, 2, SudokoValue::Two(false)).unwrap();
    sudoko.set_value(2, 1, SudokoValue::Three(false)).unwrap();
    sudoko.set_value(3, 3, SudokoValue::Four(false)).unwrap();
    sudoko.set_value(1, 5, SudokoValue::Five(false)).unwrap();
    sudoko.set_value(2, 4, SudokoValue::Six(false)).unwrap();
    sudoko.set_value(3, 6, SudokoValue::Seven(false)).unwrap();
    sudoko.set_value(1, 9, SudokoValue::Eight(false)).unwrap();
    sudoko.set_value(6, 9, SudokoValue::Nine(false)).unwrap();*/

    'main_loop: loop {

        clearscreen::clear().unwrap();

        sudoko.select_value(cursor.row, cursor.col).unwrap();

        println!("{sudoko}");
        println!("[←↑↓→] move cursor, [1-9] enter value");
        println!("[backspace] remove value");
        println!("[n] generate a new puzzle");
        println!("[l] lock values [u] unlock values");
        println!("[h] help / solve 1 step");
        println!("[s] solve the puzzle");
        println!("[q/esc] quit");
        sudoko.find_possible_values(cursor.row, cursor.col).unwrap();

        if let Err(e) = sudoko.validate() {
            println!("Validation error: {}", e.as_str().bright_red());
        }

        if let Ok(key) = stdout.read_key() {
            match key {
                console::Key::UnknownEscSeq(_) => break 'main_loop,
                console::Key::ArrowLeft => {
                    cursor.move_left();
                },
                console::Key::ArrowRight => {
                    cursor.move_right();
                },
                console::Key::ArrowUp => {
                    cursor.move_up();
                },
                console::Key::ArrowDown => {
                    cursor.move_down();
                },
                console::Key::Escape => break 'main_loop,
                console::Key::Backspace => {
                    match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Empty(true)) {
                        Err(e) => println!("{e}"),
                        Ok(_) => ()
                    }
                },
                console::Key::Char(character) => {
                    match character {
                        'q' => break 'main_loop,
                        'h' => cursor = sudoko.solve_step().unwrap(),
                        's' => sudoko.solve().unwrap(),
                        'l' => sudoko.lock().unwrap(),
                        'u' => sudoko.unlock(),
                        '1' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::One(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '2' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Two(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '3' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Three(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '4' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Four(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '5' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Five(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '6' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Six(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '7' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Seven(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '8' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Eight(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        '9' => {
                            match sudoko.set_value(cursor.row, cursor.col, SudokoValue::Nine(true)) {
                                Err(e) => println!("{e}"),
                                Ok(_) => ()
                            }
                        },
                        _ => ()
                    }
                },
                _ => (),
            }
        } 

    }

    //let value = sudoko.get_value(2, 5).unwrap();

}
