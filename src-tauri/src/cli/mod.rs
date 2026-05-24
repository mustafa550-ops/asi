/// CLI Ajanı — Komut satırı arayüzü (§9.2).
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "adler", version = "0.1.0", about = "ADLER ASI - Autonomous Digital Operator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Asimile et: GitHub reposunu analiz edip entegre et")]
    Assimilate {
        repo_url: String,
    },
    #[command(about = "Skill manifestosu yükle")]
    SkillAdd {
        file_path: String,
    },
    #[command(about = "Sistem teşhisi çalıştır")]
    Diagnostic,
}

pub fn parse() -> Option<Commands> {
    let cli = Cli::parse();
    cli.command
}
