mod commands;
mod mcp;
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
        tech: String,
    },

    /// Gera um desafio de código conforme a tecnologia e nível
    Desafio {
        /// Nome da tecnologia
        tech: String,
        /// Nível desejado (ex: iniciante, intermediario, avancado)
        level: String,
    },

    /// Cria um certificado fictício para uma trilha concluída
    Certificado {
        /// Nome da pessoa usuária
        name: String,
        /// Nome da tecnologia concluída
        tech: String,
    },

    /// Inicia o servidor MCP (Model Context Protocol) via stdio
    Mcp,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Mcp => {
            // Executa o servidor MCP diretamente
            mcp::run_mcp_server().await?;
        }
        Commands::Trilha { tech } => {
            let db = load_database();
            commands::show_trail(&db, tech);
        }
        Commands::Desafio { tech, level } => {
            let db = load_database();
            commands::show_challenge(&db, tech, level);
        }
        Commands::Certificado { name, tech } => {
            let db = load_database();
            commands::generate_certificate(&db, name, tech);
        }
    }

    Ok(())
}

/// Função auxiliar para carregar o banco de dados das trilhas
fn load_database() -> Database {
    match Database::load_from_file("data/trails.json") {
        Ok(database) => database,
        Err(err) => {
            eprintln!("Erro ao carregar data/trails.json: {}", err);
            std::process::exit(1);
        }
    }
}