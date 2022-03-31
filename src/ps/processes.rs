use super::ps_commands;
use procfs::process;
use users::get_user_by_uid;

impl ps_commands::PsCommands {
    pub(crate) fn processes_display(&self) -> Result<(), String> {
        let me = process::Process::myself().unwrap();
        let tps = procfs::ticks_per_second().unwrap();

        if self.get_is_show_details() {
            println!(
                "{: <20} {: >10} {: >10} {: <8} {: <8} {: >8} {: >8} {}",
                "USER", "PID", "VSZ", "RSS", "TTY", "START", "TIME", "CMD"
            );
        } else {
            println!("{: >5} {: <8} {: >8} {}", "PID", "TTY", "TIME", "CMD");
        }

        let tty = format!("pts/{}", me.stat.tty_nr().1);
        for prc in process::all_processes()
            .ok()
            .ok_or("Failed to get processes")?
            .iter()
        {
            let total_time = (prc.stat.utime + prc.stat.stime) as f32 / (tps as f32);
            let user = match get_user_by_uid(prc.owner) {
                Some(user) => user.name().to_str().unwrap_or("unknown").to_string(),
                None => "unknown".to_string(),
            };
            let pid = prc.stat.pid;
            let vsize = prc.stat.vsize;
            let rss = prc.stat.rss;
            let command_lien = prc
                .cmdline()
                .unwrap_or(vec!["unknown".to_string()])
                .join("");
            let start_time = prc
                .stat
                .starttime()
                .ok()
                .ok_or("Failed to get start time")?;
            let start_time = start_time.format("%H:%M").to_string();

            if self.get_is_terminal() {
                self.terminal_option_print(prc, total_time);
                continue;
            }

            if self.get_is_show_all() {
                self.all_option_print(prc, total_time);
                continue;
            }

            if self.get_is_not_terminal() {
                self.not_terminal_option_print(prc, total_time);
                continue;
            }

            if self.get_is_show_details() {
                if self.get_is_not_terminal() && prc.stat.tty_nr().0 == 0 {
                    println!(
                        "{: <20} {: >10} {: >10} {: <8} {: <8} {: >8} {: >8} {}",
                        user, pid, vsize, rss, tty, start_time, total_time, command_lien
                    );
                }

                if self.get_is_terminal() && prc.stat.tty_nr().0 != 0 {
                    println!(
                        "{: <20} {: >10} {: >10} {: <8} {: <8} {: >8} {: >8} {}",
                        user, pid, vsize, rss, tty, start_time, total_time, command_lien
                    );
                }

                if self.get_is_show_all() {
                    println!(
                        "{: <20} {: >10} {: >10} {: <8} {: <8} {: >8} {: >8} {}",
                        user, pid, vsize, rss, tty, start_time, total_time, command_lien
                    );
                }
            }

            if prc.stat.tty_nr == me.stat.tty_nr {
                if self.get_is_show_details() {
                    println!(
                        "{: <20} {: >10} {: >10} {: <8} {: <8} {: >8} {: >8} {}",
                        user, pid, vsize, rss, tty, start_time, total_time, command_lien
                    );
                } else {
                    self.no_option_print(&tty, prc, total_time);
                }
            }
        }
        return Ok(());
    }

    fn no_option_print(&self, tty: &str, prc: &process::Process, total_time: f32) {
        println!(
            "{: >5} {: <8} {: >8} {}",
            prc.stat.pid, tty, total_time, prc.stat.comm
        );
    }

    fn terminal_option_print(&self, prc: &process::Process, total_time: f32) {
        if prc.stat.tty_nr().0 == 0 {
            println!(
                "{: >5} {: <8} {: >8} {}",
                prc.stat.pid,
                prc.stat.tty_nr().1,
                total_time,
                prc.stat.comm
            );
        }
    }

    fn not_terminal_option_print(&self, prc: &process::Process, total_time: f32) {
        if prc.stat.tty_nr().0 != 0 {
            println!(
                "{: >5} {: <8} {: >8} {}",
                prc.stat.pid, "?", total_time, prc.stat.comm
            );
        }
    }

    fn all_option_print(&self, prc: &process::Process, total_time: f32) {
        let tty = if prc.stat.tty_nr().0 != 0 {
            format!("pts/{}", prc.stat.tty_nr().1)
        } else {
            "?".to_string()
        };

        println!(
            "{: >5} {: <8} {: >8} {}",
            prc.stat.pid, tty, total_time, prc.stat.comm
        );
    }
}
