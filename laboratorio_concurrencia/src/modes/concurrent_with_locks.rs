use crate::calculator::Calculator;
use crate::operation::Operation;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

pub fn run_concurrent_with_locks(inputs: impl Iterator<Item = String>) {
    let calculator = Arc::new(RwLock::new(Calculator::default()));
    let mut handles = vec![];

    for input in inputs {
        let calculator = Arc::clone(&calculator);

        let handle = thread::spawn(move || {
            let file = File::open(input).expect("failed to open input file");
            process_file_concurrent(calculator, file);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    // Solo lectura: usamos read()
    let final_value = calculator.read().unwrap().value();
    println!("(concurrente con RwLock) Resultado final: {}", final_value);
}

fn process_file_concurrent(calculator: Arc<RwLock<Calculator>>, file: File) {
    let file_reader = BufReader::new(file);

    for line in file_reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("failed to read line {}", error);
                break;
            }
        };

        apply_operation_concurrent(&calculator, &line);
    }
}

fn apply_operation_concurrent(calculator: &Arc<RwLock<Calculator>>, line: &str) {
    match Operation::from_str(line) {
        Ok(operation) => {
            let mut calc = calculator.write().unwrap(); // escritura exclusiva
            calc.apply(operation);
        }
        Err(error) => eprintln!("failed to parse line {}", error),
    };
}
