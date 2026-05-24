use super::{Agent, AgentContext};

pub struct SystemManager;

impl Agent for SystemManager {
    fn name(&self) -> String { "System Manager".into() }
    fn description(&self) -> String { "Sistem durumu izleme ve yönetim".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("sistem") || task.contains("ram") || task.contains("cpu") || task.contains("system")
    }
    fn execute(&self, task: &str, _ctx: &AgentContext) -> Result<String, String> {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let cpu_count = sys.cpus().len();
        let cpu_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / cpu_count.max(1) as f32;
        let mem_used = sys.used_memory() / 1024 / 1024;
        let mem_total = sys.total_memory() / 1024 / 1024;
        let uptime = sysinfo::System::uptime() / 60;

        Ok(format!(
            "[System Manager]\n\
             İşlemci: {} çekirdek @ %{:.1} kullanım\n\
             Bellek: {}MB / {}MB kullanımda\n\
             Çalışma süresi: {} dakika\n\
             İstenen: {}",
            cpu_count, cpu_usage, mem_used, mem_total, uptime, task
        ))
    }
}
