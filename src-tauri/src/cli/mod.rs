use clap::{Parser, Subcommand};
use std::sync::Arc;

use crate::agents::diagnostic::DiagnosticAgent;
use crate::agents::{Agent, AgentContext, ApprovalLevel};
use crate::assimilation::analyzer::RepoAnalyzer;
use crate::assimilation::registry::ModuleRegistry;
use crate::db;
use crate::llm::OllamaClient;

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
    #[command(about = "Sistem durumunu raporla")]
    Status,
}

pub fn parse() -> Option<Commands> {
    let cli = Cli::parse();
    cli.command
}

pub fn run(command: &Commands) -> Result<String, String> {
    let ollama = OllamaClient::new("http://localhost:11434".to_string());
    let ctx = AgentContext {
        ollama: &ollama,
        memory: None,
        event_bus: None,
        approval: ApprovalLevel::SemiAutonomous,
    };

    match command {
        Commands::Assimilate { repo_url } => {
            let path = format!("/tmp/adler-assimilate/{}", repo_url.replace(|c: char| !c.is_alphanumeric(), "_"));
            std::fs::create_dir_all(&path).map_err(|e| format!("Cannot create dir: {}", e))?;

            let analyzer = RepoAnalyzer::new();
            let analysis = analyzer.analyze(&path)?;
            let db_path = std::path::Path::new("adler.db");
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = ModuleRegistry::new(Arc::clone(&conn));

            registry.register(repo_url, &path, &analysis.dependencies)?;
            Ok(format!(
                "Asimilasyon başladı: {}\n  Dil: {}\n  Dosya: {}\n  Build: {}\n  Modül kaydedildi.",
                repo_url, analysis.language, analysis.file_count, analysis.build_files.join(", ")
            ))
        }
        Commands::SkillAdd { file_path } => {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| format!("Dosya okunamadı: {}", e))?;
            let db_path = std::path::Path::new("adler.db");
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let memory = crate::core::memory_manager::MemoryManager::new(conn, ollama.clone());
            memory.index_content(&content, file_path, "skill")?;
            Ok(format!("Skill manifestosu kaydedildi: {} ({} bytes)", file_path, content.len()))
        }
        Commands::Diagnostic => {
            let agent = DiagnosticAgent;
            agent.execute("sistem teşhisi", &ctx)
        }
        Commands::Status => {
            let output = std::process::Command::new("sh")
                .args(["-c", "echo 'Uptime:'; uptime; echo; echo 'Memory:'; free -h; echo; echo 'Disk:'; df -h / | tail -1"])
                .output()
                .map_err(|e| format!("Komut hatası: {}", e))?;
            let text = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(format!("[System Status]\n{}", text))
        }
    }
}
