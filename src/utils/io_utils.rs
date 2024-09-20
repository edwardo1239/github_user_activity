use std::{env, error::Error};

use crate::models::error::ArgsError;

pub fn leer_data() -> Result<String, Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err(Box::new(ArgsError::new("No se ingresó el usuario")))
    } else {
        Ok(args[1].clone())
    }
}
