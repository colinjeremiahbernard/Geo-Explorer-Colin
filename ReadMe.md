# 🧭 Geo-Explorer (Versão Rust)

> Um explorador de trilhas de aprendizagem de tecnologia construído em Rust, com suporte a desafios interativos, geração de certificados e integração com ecossistema MCP (Model Context Protocol).

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-Async-blue?style=for-the-badge)
![Serde](https://img.shields.io/badge/Serde-JSON-orange?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

---

## 📖 Sobre o Projeto

O **Geo-Explorer** é uma solução completa desenvolvida durante o desafio prático de criação de portfólio. O objetivo principal é proporcionar uma experiência rica na exploração de conhecimentos em tecnologia, geração de problemas de código dinâmicos e emissão de certificados fictícios de conclusão.

Esta implementação utiliza a linguagem **Rust** para garantir **alta performance, segurança de memória sem garbage collector** e tipagem estática rigorosa na manipulação de dados JSON.

### 🌟 Principais Funcionalidades

- **🧭 Trilha:** Exibe o plano de estudos estruturado para uma tecnologia selecionada.
- **🎯 Desafio:** Gera um problema prático de código com base no nível (iniciante, intermediário, avançado).
- **🎓 Certificado:** Emite um certificado de conclusão estilizado em arte ASCII no terminal.
- **🔌 Servidor MCP (Model Context Protocol):** Expõe as ferramentas da aplicação para consumo por assistentes virtuais e IAs externas.

---

## 🏗️ Arquitetura do Projeto

O projeto foi organizado seguindo as melhores práticas de modularização do ecossistema Rust:

```text
geo-explorer/
├── data/
│   └── trails.json          # Base de dados em formato JSON
├── src/
│   ├── main.rs              # Ponto de entrada CLI e parsing de argumentos
│   ├── models.rs            # Structs de dados e métodos de leitura JSON
│   ├── commands.rs          # Implementação das regras de negócio
│   └── mcp/                 # Módulo de integração MCP
│       └── server.rs
├── tests/                   # Testes automatizados de integração
├── Cargo.toml               # Dependências e manifesto do projeto
└── README.md
```
