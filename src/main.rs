use std::{self, error::Error, process};

use github_user_activity::utils::io_utils::leer_data;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Caused by: {}", source);
        }
        process::exit(1);
    }
}

fn run () -> Result<(), Box<dyn Error>>{
    let input = leer_data()?;
    println!("{input}");
    Ok(())
}
