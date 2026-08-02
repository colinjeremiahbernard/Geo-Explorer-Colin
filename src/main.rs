mod commands;
mod models;

use clap::{Parser, Subcommand};
use models::Database;

#[derive(Parser)]
#[command(name = "geo-explorer")]
#[command(about = "Explorador de trilhas de aprendizagem em Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apresenta o plano de estudos de acordo com a tecnologia escolhida
    Trilha { 
        /// Nome da tecnologia (ex: rust, javascript)
        tech: String 
    },
    
    /// Gera um desafio de código conforme a tecnologia e nível
    Desafio { 
        /// Nome da tecnologia
        tech: String, 
        /// Nível desejado (ex: iniciante, intermediario, avancado)
        level: String 
    },
    
    /// Cria um certificado fictício para uma trilha concluída
    Certificado { 
        /// Nome da pessoa usuária
        name: String, 
        /// Nome da tecnologia concluída
        tech: String 
    },
}

fn main() {
    let cli = Cli::parse();

    // Carrega o banco de dados do JSON
    let db = match Database::load_from_file("data/trails.json") {
        Ok(database) => database,
        Err(err) => {
            eprintln!("Erro ao carregar data/trails.json: {}", err);
            std::process::exit(1);
        }
    };

    // Executa o subcomando correto
    match &cli.command {
        Commands::Trilha { tech } => {
            commands::show_trail(&db, tech);
        }
        Commands::Desafio { tech, level } => {
            commands::show_challenge(&db, tech, level);
        }
        Commands::Certificado { name, tech } => {
            commands::generate_certificate(&db, name, tech);
        }
    }
}