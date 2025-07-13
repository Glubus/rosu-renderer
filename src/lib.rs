pub mod layout;

use crate::layout::mania::{ManiaRenderer, NoteStyle};
use rosu_map::section::general::GameMode;
use rosu_map::Beatmap;
use std::time::Instant;

pub struct Player {
    beatmap: Beatmap,
    renderer: GameModeRenderer,
    start_time: Instant,
    speed: f64,
    scroll_time_ms: f32,
}

enum GameModeRenderer {
    Mania(ManiaRenderer),
    // TODO: Ajouter d'autres modes
    // Standard(StandardRenderer),
    // Taiko(TaikoRenderer),
    // Catch(CatchRenderer),
}

impl Player {
    pub fn new(beatmap: Beatmap, column_width: f32, note_size: f32, height: f32) -> Option<Self> {
        // Créer le renderer approprié en fonction du mode de jeu
        let renderer = match beatmap.mode {
            GameMode::Mania => {
                GameModeRenderer::Mania(ManiaRenderer::with_sizes(column_width, note_size, height))
            }
            // TODO: Ajouter d'autres modes
            // GameMode::Osu => GameModeRenderer::Standard(...),
            // GameMode::Taiko => GameModeRenderer::Taiko(...),
            // GameMode::Catch => GameModeRenderer::Catch(...),
            _ => return None, // Mode non supporté
        };

        Some(Self {
            beatmap,
            renderer,
            start_time: Instant::now(),
            speed: 1.0,
            scroll_time_ms: 1000.0,
        })
    }

    pub fn set_note_style(&mut self, style: NoteStyle) {
        #[allow(irrefutable_let_patterns)]
        if let GameModeRenderer::Mania(mania) = &mut self.renderer {
            mania.set_note_style(style);
        }
    }

    pub fn get_required_size(&self) -> [f32; 2] {
        match &self.renderer {
            GameModeRenderer::Mania(mania) => {
                let keycount = self.beatmap.circle_size as usize;
                [mania.required_width(keycount), mania.required_height()]
            }
            // TODO: Ajouter d'autres modes
            // GameModeRenderer::Standard(std) => std.get_required_size(),
            // GameModeRenderer::Taiko(taiko) => taiko.get_required_size(),
            // GameModeRenderer::Catch(catch) => catch.get_required_size(),
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_scroll_time(&mut self, ms: f32) {
        self.scroll_time_ms = ms;
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.render_at(ui, egui::pos2(0.0, 0.0))
    }

    pub fn render_at(&mut self, ui: &mut egui::Ui, position: egui::Pos2) {
        let current_time = self.start_time.elapsed().as_secs_f64() * 1000.0;
        let hit_objects = &self.beatmap.hit_objects;

        match &mut self.renderer {
            GameModeRenderer::Mania(mania) => {
                let keycount = self.beatmap.circle_size as usize;
                mania.render_at(
                    ui,
                    hit_objects,
                    current_time,
                    self.scroll_time_ms,
                    self.speed,
                    keycount,
                    position,
                );
            }
            // TODO: Ajouter d'autres modes
            // GameModeRenderer::Standard(std) => std.render_at(...),
            // GameModeRenderer::Taiko(taiko) => taiko.render_at(...),
            // GameModeRenderer::Catch(catch) => catch.render_at(...),
        }
    }

    pub fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn set_current_time(&mut self, time_ms: f64) {
        self.start_time = Instant::now() - std::time::Duration::from_secs_f64(time_ms / 1000.0);
    }

    pub fn current_time(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64() * 1000.0
    }
}
