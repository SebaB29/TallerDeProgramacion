use crate::calculator::Calculator;
use crate::operation::Operation;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    sync::mpsc,
    thread,
};

pub fn run_concurrent_with_channels(inputs: impl Iterator<Item = String>) {
    let (tx, rx) = mpsc::channel::<Operation>();
    let mut handles = vec![];

    // Thread controlador que aplica operaciones
    let controller_handle = thread::spawn(move || {
        let mut calculator = Calculator::default();
        for op in rx {
            apply_operation_concurrent(&mut calculator, op);
        }
        println!(
            "(concurrente con channels) Resultado final: {}",
            calculator.value()
        );
    });

    for input in inputs {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let file = match File::open(&input) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("failed to open file {}: {}", input, e);
                    return;
                }
            };
            process_file_concurrent(tx_clone, file);
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }

    drop(tx); // Cerramos los canales para que el controlador termine
    if let Err(e) = controller_handle.join() {
        eprintln!("Controller thread panicked: {:?}", e);
    }
}

fn process_file_concurrent(tx: mpsc::Sender<Operation>, file: File) {
    let file_reader = BufReader::new(file);

    for line in file_reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("failed to read line: {}", e);
                break;
            }
        };
        apply_operation_concurrent_tx(&tx, &line);
    }
}

fn apply_operation_concurrent_tx(tx: &mpsc::Sender<Operation>, line: &str) {
    match Operation::from_str(line) {
        Ok(op) => {
            if let Err(e) = tx.send(op) {
                eprintln!("failed to send operation: {}", e);
            }
        }
        Err(e) => eprintln!("failed to parse line: {}", e),
    }
}

fn apply_operation_concurrent(calculator: &mut Calculator, operation: Operation) {
    calculator.apply(operation);
}
