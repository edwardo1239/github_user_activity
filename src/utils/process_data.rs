use std::{collections::HashMap, error::Error};
use serde_json::{self, Value};

pub struct Evento {
    pub actor: String,
    pub tipo: String,
    pub repo: String,
    pub description: String,
}

pub async fn procesar_data(data:Vec<HashMap<String, Value>>) -> Result<Vec<Evento>, Box<dyn Error>>{
    // Convertimos el Vec<HashMap<String, Value>> a un Value
    let mut eventos = Vec::new();

    for evento in data {
        let actor = evento["actor"]["login"].as_str().unwrap_or("").to_string();
        let tipo = evento["type"].as_str().unwrap_or("").to_string();
        let repo = evento["repo"]["name"].as_str().unwrap_or("").to_string();

        let description = match evento["payload"]["description"].as_str() {
            Some(desc) => desc.to_string(),
            None => evento["payload"]["commits"]
            .as_array()
            .and_then(|commits| commits.first())
            .and_then(|commit| commit["message"].as_str())
            .unwrap_or("")
            .to_string(),
        };

        eventos.push(Evento{
            actor,
            tipo,
            repo,
            description,
        });
    }
    Ok(eventos)

}

// Función auxiliar para imprimir los eventos de forma legible
pub fn imprimir_eventos(eventos: &[Evento]) {
    for (i, evento) in eventos.iter().enumerate() {
        println!("Evento {}:", i + 1);
        println!("  Actor: {}", evento.actor);
        println!("  Tipo: {}", evento.tipo);
        println!("  Repo: {}", evento.repo);
        println!("  Descripción: {}", evento.description);
        println!();
    }
}