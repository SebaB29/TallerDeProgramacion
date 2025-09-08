use crate::calculator::Calculator;
use crate::operation::Operation;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn run_sequential(inputs: impl Iterator<Item = String>) {
    // We maintain a *global* calculator for the entire program.
    let mut calculator = Calculator::default();

    for input in inputs {
        // Open the input file.
        let file = File::open(input).expect("failed to open input file");
        process_file(&mut calculator, file);
    }

    println!("{}", calculator.value())
}

fn process_file(calculator: &mut Calculator, file: File) {
    // We need to create a BufReader for the file.
    //
    // It can be excessively inefficient to work directly with a reader,
    // as each read results in a system call. A buffered readered performs
    // large, infrequent reads on the underlying reader and maintains an
    // in-memory buffer of the results.
    let file_reader = BufReader::new(file);

    // A buffered reader also implements useful methods, like `lines()`
    for line in file_reader.lines() {
        // The underlying reader (file) may fail. In that case, we print the
        // error and skip the current file.
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("failed to read line {}", error);
                break;
            }
        };

        apply_operation(calculator, &line);
    }
}

fn apply_operation(calculator: &mut Calculator, line: &str) {
    // The operation may be invalid. In that case, we print the error
    // and skip the current *line*.
    match Operation::from_str(&line) {
        Ok(operation) => calculator.apply(operation),
        Err(error) => {
            eprintln!("failed to parse line {}", error);
        }
    };
}
