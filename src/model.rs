use std::{collections::{hash_map::Values, HashMap, HashSet}, fmt::Display, io::Empty, thread, time::Duration};

use colored::Colorize;
use itertools::Itertools;

use crate::cursor::Cursor;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SudokoIndex {
    row: usize,
    col: usize,
    row_block: usize,
    col_block: usize,
    internal_row: usize,
    internal_col: usize,
    index_block: usize
}

#[allow(dead_code)]
//  Storing the data in an fixed length array
//  0 - 1 - 2
//  3 - 4 - 5
//  6 - 7 - 8
// 
#[derive(Debug, Clone)]
pub struct Sudoko {
    data: [SudokoBlock; 9],
    selected: Option<SudokoIndex>,
    lock: HashSet<(usize, usize)>
}

impl Sudoko {
    pub fn new() -> Sudoko {
        Sudoko {
            data: [SudokoBlock::new(); 9],
            selected: None,
            lock: HashSet::new()
        }
    }

    fn convert_to_index(row: usize, col: usize) -> Result<SudokoIndex, String> {
        // Validate that row is betwen 1 and 9
        let (row_block, internal_row) = match row {
            1..=3 => (1, row),
            4..=6 => (2, row-3),
            7..=9 => (3, row-6),
            _ => return Err("Row must be from 1 to 9".to_owned())
        };

        // Validate that col is betwen 1 and 9
        let (col_block, internal_col) = match col {
            1..=3 => (1, col),
            4..=6 => (2, col-3),
            7..=9 => (3, col-6),
            _ => return Err("Column must be from 1 to 9".to_owned())
        };

        let index_block = (row_block - 1) * 3 + (col_block - 1);

        Ok(SudokoIndex {
            row, col, row_block, col_block,
            internal_row, internal_col, index_block
        })
    }

    pub fn get_value(&self, row: usize, col: usize) -> Result<&SudokoValue, String> {

        let index = Sudoko::convert_to_index(row, col)?;

        Ok(self.data[index.index_block].get_value(index.internal_row, index.internal_col).unwrap())
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: SudokoValue) -> Result<(), String> {
        
        // Check if the value is locked
        if self.lock.contains(&(row, col)) {
            return Err("Value is locked!".to_string())
        }
        
        // Validate that row is betwen 1 and 9
        let index = Sudoko::convert_to_index(row, col)?;

        self.data[index.index_block].set_value(index.internal_row, index.internal_col, value)?;

        Ok(())
    }

    pub fn select_value(&mut self, row: usize, col: usize) -> Result<(), String> {
        let index = Sudoko::convert_to_index(row, col)?;

        // Unselect the old value
        if let Some(select_index) = self.selected {

            if index == select_index {
                return Ok(())
            }

            let selected_value = self.data[select_index.index_block].get_value(select_index.internal_row, select_index.internal_col)?;

            let new_value = match selected_value {
                SudokoValue::One(_) => SudokoValue::One(false),
                SudokoValue::Two(_) => SudokoValue::Two(false),
                SudokoValue::Three(_) => SudokoValue::Three(false),
                SudokoValue::Four(_) => SudokoValue::Four(false),
                SudokoValue::Five(_) => SudokoValue::Five(false),
                SudokoValue::Six(_) => SudokoValue::Six(false),
                SudokoValue::Seven(_) => SudokoValue::Seven(false),
                SudokoValue::Eight(_) => SudokoValue::Eight(false),
                SudokoValue::Nine(_) => SudokoValue::Nine(false),
                SudokoValue::Empty(_) => SudokoValue::Empty(false),
            };

            self.data[select_index.index_block].set_value(select_index.internal_row, select_index.internal_col, new_value)?;
        }

        let current_value = self.data[index.index_block].get_value(index.internal_row, index.internal_col)?;

        let new_value = match current_value {
            SudokoValue::One(_) => SudokoValue::One(true),
            SudokoValue::Two(_) => SudokoValue::Two(true),
            SudokoValue::Three(_) => SudokoValue::Three(true),
            SudokoValue::Four(_) => SudokoValue::Four(true),
            SudokoValue::Five(_) => SudokoValue::Five(true),
            SudokoValue::Six(_) => SudokoValue::Six(true),
            SudokoValue::Seven(_) => SudokoValue::Seven(true),
            SudokoValue::Eight(_) => SudokoValue::Eight(true),
            SudokoValue::Nine(_) => SudokoValue::Nine(true),
            SudokoValue::Empty(_) => SudokoValue::Empty(true),
        };

        self.data[index.index_block].set_value(index.internal_row, index.internal_col, new_value)?;

        // Set the new value as the selected
        self.selected = Some(index);

        Ok(())
    }

    pub fn unselect_value(&mut self) -> Result<(), String> {
        if let Some(select_index) = self.selected {
            let selected_value = self.data[select_index.index_block].get_value(select_index.internal_row, select_index.internal_col)?;

            let new_value = match selected_value {
                SudokoValue::One(_) => SudokoValue::One(false),
                SudokoValue::Two(_) => SudokoValue::Two(false),
                SudokoValue::Three(_) => SudokoValue::Three(false),
                SudokoValue::Four(_) => SudokoValue::Four(false),
                SudokoValue::Five(_) => SudokoValue::Five(false),
                SudokoValue::Six(_) => SudokoValue::Six(false),
                SudokoValue::Seven(_) => SudokoValue::Seven(false),
                SudokoValue::Eight(_) => SudokoValue::Eight(false),
                SudokoValue::Nine(_) => SudokoValue::Nine(false),
                SudokoValue::Empty(_) => SudokoValue::Empty(false)
            };

            self.data[select_index.index_block].set_value(select_index.internal_row, select_index.internal_col, new_value)?;
        }

        self.selected = None;

        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {

        // Check of all of the blocks
        for index_block in 0..9 {
            self.data[index_block].validate().map_err(|e| {
                match index_block {
                    0 => format!("{e} Found at top-left block!"),
                    1 => format!("{e} Found at top-center block!"),
                    2 => format!("{e} Found at top-right block!"),
                    3 => format!("{e} Found at middel-left block!"),
                    4 => format!("{e} Found at middel-center block!"),
                    5 => format!("{e} Found at middel-right block!"),
                    6 => format!("{e} Found at bottom-left block!"),
                    7 => format!("{e} Found at bottom-center block!"),
                    8 => format!("{e} Found at bottom-right block!"),
                    _ => e
                }
                
            })?
        }

        // Check each of the rows
        for row in 1..=9 {
            self.validate_row(row)?
        }

        // Check each of the columns
        for col in 1..=9 {
            self.validate_col(col)?
        }

        Ok(())
    }

    pub fn validate_row(&self, row: usize) -> Result<(), String> {
        let mut value_set: HashSet<usize> = HashSet::new();

        for col in 1..=9 {
            let index = Sudoko::convert_to_index(row, col)?;

            match self.data[index.index_block]
                    .get_value(index.internal_row, index.internal_col)?
                    .to_value() {
                None => (),
                Some(value) => {
                    if value_set.contains(&value) {
                        return Err(format!("Value '{value}' is already present in row '{row}' at column '{col}'"))
                    } else {
                        value_set.insert(value);
                    }
                }
            }
        }

        Ok(())

    }

    pub fn validate_col(&self, col: usize) -> Result<(), String> {
        let mut value_set: HashSet<usize> = HashSet::new();

        for row in 1..=9 {
            let index = Sudoko::convert_to_index(row, col)?;

            match self.data[index.index_block]
                    .get_value(index.internal_row, index.internal_col)?
                    .to_value() {
                None => (),
                Some(value) => {
                    if value_set.contains(&value) {
                        return Err(format!("Value '{value}' is already present in colum '{col}' at row '{row}'"))
                    } else {
                        value_set.insert(value);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), String> {
        
        self.solve_step()?;

        Ok(())
    }

    pub fn solve_step(&mut self) -> Result<Cursor, String> {

        let mut blocks_values: HashMap<usize, Vec<(SudokoIndex, Vec<SudokoValue>)>> = HashMap::new();

        // init hashmap
        for i in 0..9 {
            blocks_values.insert(i, Vec::new());
        }

        for row in 1..=9 {
            for col in 1..=9 {
                match self.get_value(row, col)? {
                    SudokoValue::Empty(_) => {
                        let possible_values = self.find_possible_values(row, col)?;
                        if let Some(mut values) = possible_values {
                            if values.len() == 1 {
                                if let Some(value) = values.pop() {
                                    match self.set_value(row, col, value) {
                                        Err(_) => (),
                                        Ok(_) => return Ok(Cursor {row, col})
                                    }
                                }
                            } else {
                                let index = Sudoko::convert_to_index(row, col)?;
                                if let Some(block) = blocks_values.get_mut(&index.index_block) {
                                    block.push((index, values))
                                }
                            }
                        }
                    },
                    _ => continue
                }
                
            }
        }

        // Check each block individually, and find a unique value
        for i in 0..9 {
            let mut by_value: HashMap<&SudokoValue, Vec<&SudokoIndex>> = HashMap::new();
            if let Some(block) = blocks_values.get(&i) {
                for (index, group) in block {
                    for value in group {
                        if !by_value.contains_key(&value) {
                            by_value.insert(value, Vec::new());
                        }

                        if let Some(add_value) = by_value.get_mut(&value) {
                            add_value.push(index);
                        }
                    }
                }
            }

            for (value, indexes) in by_value {
                if indexes.len() == 1 {

                    if let Some(index) = indexes.get(0) {
                        match self.set_value(index.row, index.col, *value) {
                            Err(_) => (),
                            Ok(_) => return Ok(Cursor {row: index.row, col: index.col})
                        }
                    }
                    
                }
            }

        }

        Ok(Cursor::new())
    }

    pub fn find_possible_values(&self, row: usize, col: usize) -> Result<Option<Vec<SudokoValue>>, String> {

        let index = Sudoko::convert_to_index(row, col)?;

        let mut possible_values_set = SudokoValue::full_hashset()?;

        // Check values in rows
        for row_i in 1..=9 {
            if row_i == row { continue; }
            let temp_index = Sudoko::convert_to_index(row_i, col)?;
            let value = self.data[temp_index.index_block].get_value(temp_index.internal_row, temp_index.internal_col)?;
            if possible_values_set.contains(&value) {
                possible_values_set.remove(&value);
            }
        }

        // Check values in cols
        for col_i in 1..=9 {
            if col_i == col { continue; }
            let temp_index = Sudoko::convert_to_index(row, col_i)?;
            let value = self.data[temp_index.index_block].get_value(temp_index.internal_row, temp_index.internal_col)?;
            if possible_values_set.contains(&value) {
                possible_values_set.remove(&value);
            }
        }

        // Remove values already in same block
        for value in self.data[index.index_block].get_values() {
            if possible_values_set.contains(&value) {
                possible_values_set.remove(&value);
            }
        }

        let mut possible_values: Vec<SudokoValue> = Vec::new();

        for value in possible_values_set.into_iter().sorted() {
            possible_values.push(value);
        }

        if possible_values.len() == 0 {
            // No value
            return Ok(None)
        }

        Ok(Some(possible_values))
    }

    pub fn lock(&mut self) -> Result<(), String> {

        for row in 1..=9 {
            for col in 1..=9 {
                let index = Sudoko::convert_to_index(row, col)?;

                let value = self.data[index.index_block].get_value(index.internal_row, index.internal_col)?;

                match value {
                    SudokoValue::Empty(_) => (),
                    _ => {
                        // Add value into lock
                        self.lock.insert((row, col));
                    }
                }

            }
        }

        Ok(())
    }

    pub fn is_locked(&self, row: usize, col: usize) -> bool {
        self.lock.contains(&(row, col))
    }

    pub fn unlock(&mut self) {
        self.lock.clear()
    }

}

impl Display for Sudoko {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for row in 1..=9 {
            for row_height in 1..=2 {
                match row_height {
                    1 => match row {
                        1 => writeln!(f, "┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓")?,
                        2 => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                        3 => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                        4 => writeln!(f, "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫")?,
                        5 => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                        6 => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                        7 => writeln!(f, "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫")?,
                        8 => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                        9 => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                        _ => writeln!(f, "")?
                    }
                    2 => {
                        for col in 1..=9 {
                            match col {
                                1 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "┃ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "┃ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                2 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "│ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "│ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                3 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "│ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "│ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                4 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "┃ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "┃ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                5 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "│ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "│ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                6 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "│ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "│ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                7 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "┃ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "┃ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                8 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "│ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "│ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                9 => {
                                    match self.is_locked(row, col) {
                                        false => write!(f, "│ {} ", self.get_value(row, col).unwrap())?,
                                        true => write!(f, "│ {} ", self.get_value(row, col).unwrap()
                                            .to_string().as_str().underline())?
                                    }
                                },
                                _ => writeln!(f, "")?
                            }
                        }
                        writeln!(f, "┃")?;
                    },
                    _ => ()
                }
            }
        }

        write!(f, "┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛")

    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SudokoBlock {
    data: [SudokoValue; 9]
}

impl SudokoBlock {
    fn new() -> SudokoBlock {

        SudokoBlock {
            data: [SudokoValue::new(None).unwrap(); 9]
        }

    }

    pub fn convert_to_index(&self, row: usize, col: usize) -> Result<SudokoIndex, String> {
        match row {
            1..=3 => (),
            _ => return Err("Row must be from 1 to 3".to_owned())
        }

        // Validate that row is betwen 1 and 3
        match col {
            1..=3 => (),
            _ => return Err("Colum must be from 1 to 3".to_owned())
        }

        let index_block = (row - 1) * 3 + (col - 1);

        Ok(SudokoIndex {
            row, col, 
            row_block: row, col_block: col,
            internal_row: row, internal_col: col,
            index_block
        })
    }

    pub fn get_value(&self, row: usize, col: usize) -> Result<&SudokoValue, String> {
        let index = self.convert_to_index(row, col)?;

        Ok(&self.data[index.index_block])
    }

    pub fn get_values(&self) -> Vec<SudokoValue> {
        
        let mut values: Vec<SudokoValue> = Vec::new();

        for value in self.data {
            match value {
                SudokoValue::One(_) => values.push(SudokoValue::One(false)),
                SudokoValue::Two(_) => values.push(SudokoValue::Two(false)),
                SudokoValue::Three(_) => values.push(SudokoValue::Three(false)),
                SudokoValue::Four(_) => values.push(SudokoValue::Four(false)),
                SudokoValue::Five(_) => values.push(SudokoValue::Five(false)),
                SudokoValue::Six(_) => values.push(SudokoValue::Six(false)),
                SudokoValue::Seven(_) => values.push(SudokoValue::Seven(false)),
                SudokoValue::Eight(_) => values.push(SudokoValue::Eight(false)),
                SudokoValue::Nine(_) => values.push(SudokoValue::Nine(false)),
                SudokoValue::Empty(_) => (),
            }
        }

        values

    }

    pub fn set_value(&mut self, row: usize, col: usize, value: SudokoValue) -> Result<(), String> {
        let index = self.convert_to_index(row, col)?;

        self.data[index.index_block] = value;

        Ok(())
    }
    
    pub fn validate(&self) -> Result<(), String> {
        let mut value_set: HashSet<usize> = HashSet::new();

        for value in self.data {
            match value.to_value() {
                None => (),
                Some(value) => {
                    if value_set.contains(&value) {
                        return Err(format!("Value {value} is already present!"))
                    } else {
                        value_set.insert(value);
                    }
                }
            }
        }

        Ok(())
    }

}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SudokoValue {
    One(bool), Two(bool), Three(bool),
    Four(bool), Five(bool), Six(bool),
    Seven(bool), Eight(bool), Nine(bool),
    Empty(bool)
}

impl SudokoValue {
    pub fn new(value: Option<usize>) -> Result<SudokoValue, String> {
        match value {
            None => Ok(SudokoValue::Empty(false)),
            Some(x) => match x {
                0 => Ok(SudokoValue::Empty(false)),
                1 => Ok(SudokoValue::One(false)),
                2 => Ok(SudokoValue::Two(false)),
                3 => Ok(SudokoValue::Three(false)),
                4 => Ok(SudokoValue::Four(false)),
                5 => Ok(SudokoValue::Five(false)),
                6 => Ok(SudokoValue::Six(false)),
                7 => Ok(SudokoValue::Seven(false)),
                8 => Ok(SudokoValue::Eight(false)),
                9 => Ok(SudokoValue::Nine(false)),
                _ => Err("Value must be None or 1-9".to_owned())
            }
            
        }
    }

    pub fn full_hashset() -> Result<HashSet<SudokoValue>, String> {
        let mut set: HashSet<SudokoValue> = HashSet::new();
        for i in 1..=9 {
            set.insert(SudokoValue::new(Some(i))?);
        }
        Ok(set)
    }

    pub fn to_value(&self) -> Option<usize> {
        match self {
            SudokoValue::One(_) => Some(1),
            SudokoValue::Two(_) => Some(2),
            SudokoValue::Three(_) => Some(3),
            SudokoValue::Four(_) => Some(4),
            SudokoValue::Five(_) => Some(5),
            SudokoValue::Six(_) => Some(6),
            SudokoValue::Seven(_) => Some(7),
            SudokoValue::Eight(_) => Some(8),
            SudokoValue::Nine(_) => Some(9),
            SudokoValue::Empty(_) => None
        }
    }
}

impl Display for SudokoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokoValue::One(selected) => {
                match selected {
                    false => write!(f, "{}", "1".bright_blue().dimmed()),
                    true => write!(f, "{}", "1".bright_blue().bold()),
                }
            },
            SudokoValue::Two(selected) => {
                match selected {
                    false => write!(f, "{}", "2".bright_cyan().dimmed()),
                    true => write!(f, "{}", "2".bright_cyan().bold())
                }
            },
            SudokoValue::Three(selected) => {
                match selected {
                    false => write!(f, "{}", "3".bright_green().dimmed()),
                    true => write!(f, "{}", "3".bright_green().bold())
                }
            },
            SudokoValue::Four(selected) => {
                match selected {
                    false => write!(f, "{}", "4".bright_magenta().dimmed()),
                    true => write!(f, "{}", "4".bright_magenta().bold())
                }
            },
            SudokoValue::Five(selected) => {
                match selected {
                    false => write!(f, "{}", "5".bright_purple().dimmed()),
                    true => write!(f, "{}", "5".bright_purple().bold())
                }
            },
            SudokoValue::Six(selected) => {
                match selected {
                    false => write!(f, "{}", "6".bright_red().dimmed()),
                    true => write!(f, "{}", "6".bright_red().bold()),
                }
            },
            SudokoValue::Seven(selected) => {
                match selected {
                    false => write!(f, "{}", "7".bright_yellow().dimmed()),
                    true => write!(f, "{}", "7".bright_yellow().bold())
                }
            },
            SudokoValue::Eight(selected) => {
                match selected {
                    false => write!(f, "{}", "8".bright_blue().dimmed()),
                    true => write!(f, "{}", "8".bright_blue().bold())
                }
            },
            SudokoValue::Nine(selected) => {
                match selected {
                    false => write!(f, "{}", "9".bright_red().dimmed()),
                    true => write!(f, "{}", "9".bright_red().bold())
                }
            },
            SudokoValue::Empty(selected) => {
                match selected {
                    false => write!(f, " "),
                    true => write!(f, "{}", " ".on_white())
                }
            },
        }
    }
}