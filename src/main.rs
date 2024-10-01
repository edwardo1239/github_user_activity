use std::{self, error::Error, process};
use tokio;
use github_user_activity::utils::{connection_utils::enviar_solicitur, io_utils::leer_data, process_data::{imprimir_eventos, procesar_data}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Caused by: {}", source);
        }
        process::exit(1);
    }

    Ok(())

}

async fn run () -> Result<(), Box<dyn Error>>{
    let input = leer_data()?;
    let response = enviar_solicitur(&input).await?;
    let data = procesar_data(response).await?;
    imprimir_eventos(&data);
    Ok(())
}
