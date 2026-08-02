use crate::models::Database;
use colored::*;

/// Exibe o plano de estudos de uma tecnologia
pub fn show_trail(db: &Database, tech_key: &str) {
    if let Some(tech) = db.techs.get(tech_key) {
        println!("\n{}", format!("=== Trilha: {} ===", tech.name).bold().cyan());
        
        println!("\n{}", "📌 Níveis Disponíveis:".yellow());
        for level in &tech.levels {
            println!("  • {}", level);
        }

        println!("\n{}", "📚 Módulos do Curso:".green());
        for module in &tech.modules {
            println!("  {}", module);
        }
        println!();
    } else {
        eprintln!("{}", format!("❌ Trilha para '{}' não encontrada.", tech_key).red());
    }
}

/// Gera um desafio de código conforme a tecnologia e nível
pub fn show_challenge(db: &Database, tech_key: &str, level: &str) {
    if let Some(tech) = db.techs.get(tech_key) {
        if let Some(challenge) = tech.challenges.get(level) {
            println!("\n{}", format!("🎯 Desafio: {} [{}]", tech.name, level.to_uppercase()).bold().yellow());
            println!("\n{}", challenge.bold());
            println!();
        } else {
            eprintln!("{}", format!("❌ Nível '{}' não encontrado para a trilha '{}'.", level, tech_key).red());
        }
    } else {
        eprintln!("{}", format!("❌ Tecnologia '{}' não encontrada.", tech_key).red());
    }
}

/// Gera um certificado fictício em ASCII
pub fn generate_certificate(db: &Database, name: &str, tech_key: &str) {
    if let Some(tech) = db.techs.get(tech_key) {
        let title = format!(" CERTIFICADO DE CONCLUSAO ");
        let name_line = format!(" Certificamos que {} ", name);
        let course_line = format!(" concluiu com exito a trilha de {} ", tech.name);

        println!("\n{}", "┌────────────────────────────────────────────────────────┐".bright_yellow());
        println!("{}", format!("│{:^56}│", title).bright_yellow().bold());
        println!("{}", "├────────────────────────────────────────────────────────┤".bright_yellow());
        println!("{}", "│                                                        │".bright_yellow());
        println!("{}", format!("│{:^56}│", name_line).bold().white());
        println!("{}", "│                                                        │".bright_yellow());
        println!("{}", format!("│{:^56}│", course_line).cyan());
        println!("{}", "│                                                        │".bright_yellow());
        println!("{}", "│                       GEO-EXPLORER (VERSAO RUST)       │".bright_yellow());
        println!("{}", "└────────────────────────────────────────────────────────┘\n".bright_yellow());
    } else {
        eprintln!("{}", format!("❌ Não foi possível gerar certificado. Trilha '{}' não existe.", tech_key).red());
    }
}