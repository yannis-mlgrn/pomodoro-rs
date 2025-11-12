use eframe::egui;
use env_logger::init;
use instant::{Duration, Instant}; 

struct TimeSetting{
    pause: f64,
    work: f64
}

#[derive(PartialEq, Debug, Clone, Copy)] // Ajouter Clone et Copy pour l'état simple
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

    // Réglages par défaut (20 minutes de travail, 5 minutes de pause)
    let settings_config: TimeSetting = TimeSetting{work:20.0,pause:5.0}; 

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(380.0, 200.0)),
        ..Default::default()
    };

    // Initialisation des variables d'état
    let mut status = TimerState::Idle;
    let mut start_instant = Instant::now(); // Sera réinitialisé au démarrage
    // Calcul de la durée initiale de travail en Duration
    let mut total_duration = Duration::from_secs_f64(settings_config.work * 60.0);
    let initial_remaining = total_duration;

    eframe::run_simple_native("Pomodoro", options, move |ctx, _frame| {
        let mut local_status = status;
        let mut local_start_instant = start_instant;

        let mut remaining = initial_remaining;

        if local_status == TimerState::Working {
            
            total_duration = Duration::from_secs_f64(settings_config.work * 60.0);
            let elapsed = Instant::now() - local_start_instant;
            remaining = total_duration.checked_sub(elapsed).unwrap_or(Duration::ZERO);
            
            if remaining == Duration::ZERO {
                local_status = TimerState::Paused;
                // Démarre immédiatement le compteur de pause
                local_start_instant = Instant::now();
            } else {
                ctx.request_repaint_after(Duration::from_secs_f32(0.03));
            }
        }else if local_status == TimerState::Paused {
            
            let elapsed = Instant::now() - local_start_instant;
            total_duration = Duration::from_secs_f64(settings_config.pause * 60.0);

            remaining = total_duration.checked_sub(elapsed).unwrap_or(Duration::ZERO);
            
            if remaining == Duration::ZERO {
                local_status = TimerState::Idle;
            } else {
                ctx.request_repaint_after(Duration::from_secs_f32(0.03));
            }
        }

        // --- DESSIN DE L'INTERFACE UTILISATEUR ---
egui::CentralPanel::default().show(ctx, |ui| {
    ui.add_space(5.0);

    if local_status == TimerState::Working {
        // Le titre et le compteur sont déjà centrés par vertical_centered_justified
        ui.vertical_centered_justified(|ui| {
            ui.heading("Session de travail");
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
        
        // CORRECTION DE CENTRAGE : Utiliser ui.vertical_centered() pour centrer le bloc horizontal
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                if ui.button("🔄 Reset").clicked() {
                    // Reset à l'état Idle et temps de travail initial
                    remaining = initial_remaining;
                    local_status = TimerState::Idle;
                    local_start_instant = Instant::now();
                }
                ui.add_space(10.0); // Espace entre les boutons
                if ui.button("▶ Passer").clicked() {
                    // Pour passer au prochain état (Pause)
                    local_status = TimerState::Paused;
                    local_start_instant = Instant::now(); 
                }
            });
        });
        
        // FIN du bloc 'if local_status == TimerState::Working'
    }
    else if local_status == TimerState::Paused {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Pause");
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
        
        // Centrage des boutons de pause
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                if ui.button("🔄 Reset").clicked() {
                    remaining = initial_remaining;
                    local_status = TimerState::Idle;
                }
                ui.add_space(10.0);
                if ui.button("▶ Passer").clicked() {
                    // Pour passer à l'état Working
                    local_status = TimerState::Working;
                    local_start_instant = Instant::now();
                }
            });
        });
        // FIN du bloc 'else if local_status == TimerState::Paused'

    } else {
        // Bloc 'else' final (local_status == TimerState::Idle)
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
        
        // Centrage du bouton Démarrer
        ui.vertical_centered(|ui| {
            if ui.button("Démarrer la session de travail").clicked() {
                local_status = TimerState::Working;
                local_start_instant = Instant::now();
            }
        });
    } 
    // FIN de la structure if/else if/else

    ui.add_space(5.0); // Espace final

});
// FIN de egui::CentralPanel::default().show

        status = local_status;
        start_instant = local_start_instant;

    })
}