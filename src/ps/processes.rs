use super::ps_commands;
use procfs::process;

impl ps_commands::PsCommands {
    pub(crate) fn processes_display(&self) -> Result<(), String> {
        let me = process::Process::myself().unwrap();
        let tps = procfs::ticks_per_second().unwrap();

        println!("{: >5} {: <8} {: >8} {}", "PID", "TTY", "TIME", "CMD");

        let tty = format!("pts/{}", me.stat.tty_nr().1);
        for prc in process::all_processes()
            .ok()
            .ok_or("Failed to get processes")?
            .iter()
        {
            if prc.stat.tty_nr == me.stat.tty_nr {
                let total_time = (prc.stat.utime + prc.stat.stime) as f32 / (tps as f32);
                println!(
                    "{: >5} {: <8} {: >8} {}",
                    prc.stat.pid, tty, total_time, prc.stat.comm
                );
            }
        }
        return Ok(());
    }
}
