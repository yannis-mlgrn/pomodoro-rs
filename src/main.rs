use eframe::egui;
use instant::{Duration, Instant}; 

struct PauseSetting {
    duration: f64,
    title: String,
    next: TimerState
}
struct WorkSetting {
    duration: f64,
    title: String,
    next: TimerState
}
struct TimeSetting {
    pause: PauseSetting,
    work: WorkSetting,
}

trait TimerSetting {
    fn get_title(&self) -> &str;
    fn get_next_status(&self) -> TimerState;
}

impl TimerSetting for WorkSetting {
    fn get_title(&self) -> &str {
        &self.title
    }
    fn get_next_status(&self) -> TimerState {
        self.next
    }
}

impl TimerSetting for PauseSetting {
    fn get_title(&self) -> &str {
        &self.title
    }
    fn get_next_status(&self) -> TimerState {
        self.next
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum TimerState {
    Working,
    Paused,
    Idle,
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs_f64(); 
    let minutes = (total_seconds / 60.0).floor();  
    let seconds = total_seconds % 60.0;
    format!("{:02.0}:{:02.0}", minutes, seconds.floor())
}

fn main() -> eframe::Result {
    env_logger::init();

    let settings = TimeSetting {
    work: WorkSetting { duration: 25.0, title: String::from("Productivity Mode 🚀"), next: TimerState::Paused },
    pause: PauseSetting { duration: 5.0, title: String::from("Coffee Break ☕"), next: TimerState::Working },
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(380.0, 200.0)),
        ..Default::default()
    };

    let mut status = TimerState::Idle;
    let mut start_instant = Instant::now();
    let mut total_duration = Duration::from_secs_f64(settings.work.duration * 60.0);
    let initial_remaining = total_duration;

    eframe::run_simple_native("Pomodoro", options, move |ctx, _frame| {
        let mut local_status = status;
        let mut local_start_instant = start_instant;

        let mut remaining = initial_remaining;

        if local_status == TimerState::Working {
            total_duration = Duration::from_secs_f64(settings.work.duration * 60.0);
            let elapsed = Instant::now() - local_start_instant;
            remaining = total_duration.checked_sub(elapsed).unwrap_or(Duration::ZERO);
            
            if remaining == Duration::ZERO {
                local_status = TimerState::Paused;
                local_start_instant = Instant::now();
            } else {
                ctx.request_repaint_after(Duration::from_secs_f32(0.03));
            }
        }else if local_status == TimerState::Paused {
            
            let elapsed = Instant::now() - local_start_instant;
            total_duration = Duration::from_secs_f64(settings.pause.duration * 60.0);

            remaining = total_duration.checked_sub(elapsed).unwrap_or(Duration::ZERO);
            
            if remaining == Duration::ZERO {
                local_status = TimerState::Idle;
            } else {
                ctx.request_repaint_after(Duration::from_secs_f32(0.03));
            }
        }

egui::CentralPanel::default().show(ctx, |ui| {
    ui.add_space(5.0);

    if local_status == TimerState::Idle {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Prêt à Démarrer");
            ui.separator();
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(format_duration(initial_remaining))
                    .font(egui::FontId::proportional(60.0))
                    .monospace()
            );
            ui.add_space(10.0);
            ui.separator();
        });
        
        ui.vertical_centered(|ui| {
            if ui.button("Démarrer la session de travail").clicked() {
                local_status = TimerState::Working;
                local_start_instant = Instant::now();
            }
        });
    }else {
        let current_setting: &dyn TimerSetting = if local_status == TimerState::Working {
            &settings.work
        } else {
            &settings.pause
        };
        ui.vertical_centered_justified(|ui| {
            ui.heading(current_setting.get_title());
            ui.separator();
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(format_duration(remaining))
                    .font(egui::FontId::proportional(60.0))
                    .monospace()
            );
        });
        
        ui.add_space(10.0);
        ui.separator();
        
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                if ui.button("🔄 Reset").clicked() {
                    remaining = initial_remaining;
                    local_status = TimerState::Idle;
                    local_start_instant = Instant::now();
                }
                ui.add_space(10.0);
                if ui.button("▶ Passer").clicked() {
                    local_status = current_setting.get_next_status();
                    local_start_instant = Instant::now(); 
                }
            });
        });
        
    } 

    ui.add_space(5.0);

    });
        status = local_status;
        start_instant = local_start_instant;

    })
}