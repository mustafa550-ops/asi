use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};
use std::sync::Arc;

use crate::agents::diagnostic::DiagnosticAgent;
use crate::agents::{Agent, AgentContext};
use crate::assimilation::pipeline::AssimilationPipeline;
use crate::assimilation::registry::ModuleRegistry;
use crate::config::AppConfig;
use crate::db;
use crate::llm::OllamaClient;
use crate::skill::executor::SkillExecutor;
use crate::skill::parser::ManifestoParser;
use crate::skill::registry::SkillRegistry;

#[derive(Parser)]
#[command(name = "adler", version = "0.3.0", about = "ADLER ASI - Autonomous Digital Operator (headless)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Asimile et: GitHub reposunu klonla, analiz et, entegre et")]
    Assimilate {
        repo_url: String,
    },
    #[command(about = "Skill manifestosu yükle (.md)")]
    SkillAdd {
        file_path: String,
    },
    #[command(about = "Sistem teşhisi çalıştır")]
    Diagnostic,
    #[command(about = "Sistem durumunu raporla")]
    Status,
    #[command(about = "Skill'leri listele (aktif/pasif)")]
    SkillList,
    #[command(about = "Skill'i devre dışı bırak")]
    SkillDeactivate {
        name: String,
    },
    #[command(about = "Skill'i aktifleştir")]
    SkillActivate {
        name: String,
    },
    #[command(about = "Skill'i çalıştır (manuel trigger)")]
    SkillRun {
        name: String,
        task: String,
    },
    #[command(about = "Skill'i sil")]
    SkillRemove {
        name: String,
    },
    #[command(about = "Guvenlik denetimi yap")]
    SecurityAudit,

    #[command(about = "Interaktif sohbet modu (REPL)")]
    Chat,
}

pub fn run_from_args() -> Result<String, String> {
    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => run(&cmd),
        None => Ok("ADLER ASI — use --help for available commands".into()),
    }
}

fn run(command: &Commands) -> Result<String, String> {
    let config = AppConfig::load();
    let ollama = OllamaClient::new(config.ollama_url.clone(), config.ollama_model.clone());
    let ctx = AgentContext {
        ollama: &ollama,
        claude: None,
        memory: None,
        event_bus: None,
        approval: config.resolve_approval_level(),
        vosk_model_path: "",
    };

    match command {
        Commands::Assimilate { repo_url } => {
            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = ModuleRegistry::new(Arc::clone(&conn));
            let modules_base = "modules";

            let pipeline = AssimilationPipeline::new();
            let report = pipeline.run(repo_url, &registry, modules_base);

            if report.success {
                let konum = report.module_path.clone().unwrap_or_default();
                Ok(format!(
                    "{}\n  Adımlar: {}\n  Modül yolu: {}",
                    report.summary,
                    report.steps_completed.join(" → "),
                    konum,
                ))
            } else {
                Err(format!(
                    "{}\n  Tamamlanan adımlar: {}\n  Hatalar: {}",
                    report.summary,
                    report.steps_completed.join(", "),
                    report.errors.join("; "),
                ))
            }
        }
        Commands::SkillAdd { file_path } => {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| format!("Dosya okunamadı: {}", e))?;

            let manifesto = ManifestoParser::parse(&content, file_path)?;
            let steps = ManifestoParser::manifesto_to_steps(&manifesto);
            let triggers = manifesto.triggers.clone();
            let evolution = manifesto.evolution.clone();

            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = SkillRegistry::new(conn);

            let id = registry.register(
                &manifesto.name,
                &manifesto.description,
                &triggers,
                &manifesto.approval,
                &steps,
                manifesto.logic.as_deref(),
                &evolution,
            )?;

            let memory = crate::core::memory_manager::MemoryManager::new(
                registry.conn_clone(), ollama.clone());
            memory.index_content(&content, file_path, "skill_manifesto")?;

            Ok(format!(
                "Skill manifestosu kaydedildi: '{}' (id={}, trigger={}, adım={})",
                manifesto.name, id, triggers.len(), steps.len()
            ))
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
            let sys_text = String::from_utf8_lossy(&output.stdout).to_string();

            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = crate::assimilation::registry::ModuleRegistry::new(std::sync::Arc::clone(&conn));
            let modules = registry.list_all();
            let skill_reg = SkillRegistry::new(db::open(db_path).map_err(|e| e.to_string())?);
            let skills = skill_reg.list().unwrap_or_default();

            Ok(format!("[System Status]\n{}---\nKayıtlı Modüller ({}):\n{}\n---\nSkill'ler ({}):\n{}",
                sys_text,
                modules.len(),
                modules.iter().map(|m| format!("  - {} (path: {})", m.name, m.path)).collect::<Vec<_>>().join("\n"),
                skills.len(),
                skills.iter().map(|s| format!("  - {} {} v{} (run: {})", s.name, if s.active { "🟢" } else { "🔴" }, s.version, s.run_count)).collect::<Vec<_>>().join("\n"),
            ))
        }
        Commands::SkillList => {
            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = SkillRegistry::new(conn);
            let skills = registry.list()?;
            if skills.is_empty() {
                return Ok("Kayitli skill bulunamadi.".into());
            }
            Ok(skills.iter().map(|s| {
                format!("{:20} | {:8} | v{} | {:5} run | tetikleyiciler: {}",
                    s.name, if s.active { "AKTIF" } else { "PASIF" }, s.version, s.run_count, s.triggers.join(", "))
            }).collect::<Vec<_>>().join("\n"))
        }
        Commands::SkillDeactivate { name } => {
            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = SkillRegistry::new(conn);
            registry.deactivate(&name)?;
            Ok(format!("Skill '{}' pasiflestirildi.", name))
        }
        Commands::SkillActivate { name } => {
            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = SkillRegistry::new(conn);
            registry.activate(&name)?;
            Ok(format!("Skill '{}' aktiflestirildi.", name))
        }
        Commands::SkillRun { name, task } => {
            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = SkillRegistry::new(conn);
            let skill = registry.get_by_name(&name)?
                .ok_or_else(|| format!("Skill '{}' bulunamadi.", name))?;
            let executor = SkillExecutor::new();
            let memory = crate::core::memory_manager::MemoryManager::new(
                registry.conn_clone(), ollama.clone());
            let result = executor.execute(&skill, &task, &ollama, Some(&memory))?;
            registry.increment_run_count(&skill.name).ok();
            Ok(format!("{}\n{}", result.summary, result.step_results.join("\n")))
        }
        Commands::SkillRemove { name } => {
            let db_path = std::path::Path::new(&config.db_path);
            let conn = db::open(db_path).map_err(|e| e.to_string())?;
            let registry = SkillRegistry::new(conn);
            registry.remove(&name)?;
            Ok(format!("Skill '{}' silindi.", name))
        }
        Commands::Chat => {
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            println!("ADLER ASI — Sohbet Modu ({} /exit ile cikis)", ctx.ollama.model());
            println!("{}", "-".repeat(40));

            let mut context: Vec<String> = Vec::new();
            loop {
                print!("> ");
                stdout.flush().map_err(|e| format!("Cikis hatasi: {}", e))?;

                let mut line = String::new();
                match stdin.lock().read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(e) => {
                        if e.kind() == io::ErrorKind::Interrupted {
                            break;
                        }
                        return Err(format!("Okuma hatasi: {}", e));
                    }
                }

                let line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }
                if line == "/exit" || line == "/quit" {
                    println!("Gorusmek uzere.");
                    break;
                }
                if line == "/help" {
                    println!("Komutlar: /exit, /quit, /help, /clear");
                    continue;
                }
                if line == "/clear" {
                    context.clear();
                    println!("Hafiza temizlendi.");
                    continue;
                }

                let full_prompt = if context.is_empty() {
                    line.clone()
                } else {
                    format!("{}\nKullanici: {}", context.join("\n"), line)
                };

                match ctx.ollama.generate_sync(&full_prompt) {
                    Ok(response) => {
                        println!("Adler: {}", response);
                        context.push(format!("Kullanici: {}", line));
                        context.push(format!("Adler: {}", response));
                        if context.len() > 20 {
                            context.drain(0..2);
                        }
                    }
                    Err(e) => {
                        eprintln!("Hata: {}", e);
                    }
                }
            }
            Ok("Sohbet sonlandi.".into())
        }
        Commands::SecurityAudit => {
            let config_findings = crate::security::SecurityAuditor::audit_config(&config);
            let env_findings = crate::security::SecurityAuditor::check_env_secrets();

            let mut report = vec!["=== Guvenlik Denetimi ===".into()];
            report.push("-- Yapilandirma --".into());
            report.extend(config_findings);
            report.push("-- Ortam Degiskenleri --".into());
            if env_findings.is_empty() {
                report.push("  Ortamda API anahtari/token bulunamadi.".into());
            } else {
                report.extend(env_findings);
            }
            report.push("-- Veri Tabani --".into());
            report.push(format!("  DB yolu: {}", config.db_path));
            if config.db_path.ends_with(".db") || config.db_path.ends_with(".sqlite") {
                report.push("  UYARI: DB sifrelenmemis (SQLCipher kullanilabilir)".into());
            }

            Ok(report.join("\n"))
        }
    }
}
