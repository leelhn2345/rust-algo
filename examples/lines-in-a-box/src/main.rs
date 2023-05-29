use std::{error::Error, io::stdin};

fn num_of_lines_in_box(cols: u32, rows: u32) -> u32 {
    if cols < 3 || rows < 3 {
        return 0;
    }

    let horizontal_lines = total_num_of_horizontal_lines(cols, rows);

    let vertical_lines = total_num_of_vertical_lines(cols, rows);

    let diagonal_lines = total_num_of_diagonal_lines(cols, rows);

    horizontal_lines + vertical_lines + diagonal_lines
}

fn total_num_of_horizontal_lines(cols: u32, rows: u32) -> u32 {
    (cols - 2) * rows
}

fn total_num_of_vertical_lines(cols: u32, rows: u32) -> u32 {
    (rows - 2) * cols
}

fn total_num_of_diagonal_lines(cols: u32, rows: u32) -> u32 {
    2 * (cols - 2) * (rows - 2)
}

fn load_params() -> Result<(u32, u32), Box<dyn Error>> {
    let mut col_string = String::new();

    println!("enter number of columns");
    stdin()
        .read_line(&mut col_string)
        .expect("cannot read line");

    let cols = col_string.trim().parse::<u32>()?;

    if !(0..=1_000).contains(&cols) {
        return Err("columns must be between range of 0 to 1000 inclusive".into());
    }

    let mut row_string = String::new();
    println!("enter number of rows");
    stdin()
        .read_line(&mut row_string)
        .expect("cannot read line");

    let rows: u32 = row_string.trim().parse()?;

    if !(0..=1_000).contains(&rows) {
        return Err("rows must be between range of 0 to 1000 inclusive".into());
    }

    Ok((cols, rows))
}

fn main() {
    println!("Hello, world!");
    loop {
        let (cols, rows) = match load_params() {
            Ok((x, y)) => (x, y),
            Err(err) => {
                println!("{}\n", err);
                continue;
            }
        };
        let num_of_lines = num_of_lines_in_box(cols, rows);
        println!("The number of possible lines: {}\n", num_of_lines);
        continue;
    }
}
