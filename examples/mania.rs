use eframe::egui::{self, Color32};
use egui::{ImageSource, Vec2};
use rfd::FileDialog;
use rosu_layout::{
    layout::mania::{NoteShape, NoteStyle},
    Player,
};
use rosu_map::Beatmap;
use std::path::PathBuf;

struct ManiaApp {
    player: Player,
    playback_time: f64,
    map_duration: f64,
    playback_speed: f64,
    scroll_speed: f32,
    note_style_idx: usize,
    last_error: Option<String>,
    note_color: Color32,
    hold_body_color: Color32,
    hold_cap_color: Color32,
    column_width: f32,
    note_size: f32,
    beatmap_ln: Beatmap,
    beatmap_normal: Beatmap,
    using_ln: bool,
}

impl ManiaApp {
    fn new(
        beatmap_ln: Beatmap,
        beatmap_normal: Beatmap,
        column_width: f32,
        note_size: f32,
        height: f32,
    ) -> Option<Self> {
        // Calculate total map duration including hold notes
        let map_duration = beatmap_ln
            .hit_objects
            .iter()
            .map(|obj| match &obj.kind {
                rosu_map::section::hit_objects::HitObjectKind::Hold(h) => {
                    obj.start_time + h.duration
                }
                _ => obj.start_time,
            })
            .fold(0.0, f64::max);

        Player::new(beatmap_ln.clone(), column_width, note_size, height).map(|mut player| {
            let note_color = Color32::from_rgb(0, 174, 255);
            let hold_body_color = Color32::from_rgb(200, 200, 200);
            let hold_cap_color = Color32::from_rgb(0, 174, 255);

            let initial_style = NoteStyle {
                shape: NoteShape::Circle,
                color: note_color,
                hold_body_color,
                hold_cap_color,
            };

            player.set_note_style(initial_style);

            Self {
                player,
                playback_time: 0.0,
                map_duration,
                playback_speed: 1.0,
                scroll_speed: 1000.0,
                note_style_idx: 0,
                last_error: None,
                note_color,
                hold_body_color,
                hold_cap_color,
                column_width,
                note_size,
                beatmap_ln,
                beatmap_normal,
                using_ln: true,
            }
        })
    }

    fn reload_player(&mut self) {
        self.reload_player_with_reset(true);
    }

    fn reload_player_with_reset(&mut self, should_reset: bool) {
        let beatmap = if self.using_ln {
            self.beatmap_ln.clone()
        } else {
            self.beatmap_normal.clone()
        };

        // Update map duration for the new map
        self.map_duration = beatmap
            .hit_objects
            .iter()
            .map(|obj| match &obj.kind {
                rosu_map::section::hit_objects::HitObjectKind::Hold(h) => {
                    obj.start_time + h.duration
                }
                _ => obj.start_time,
            })
            .fold(0.0, f64::max);

        // Reset playback time only when switching maps
        if should_reset {
            self.playback_time = 0.0;
        }

        self.player = Player::new(beatmap, self.column_width, self.note_size, 800.0)
            .expect("Failed to create player");
        self.player
            .set_note_style(self.get_note_style(self.note_style_idx));
        self.player.set_current_time(self.playback_time);
        self.player.set_speed(self.playback_speed);
        self.player.set_scroll_time(self.scroll_speed);
    }

    fn get_note_style(&self, idx: usize) -> NoteStyle {
        match idx {
            0 => NoteStyle {
                shape: NoteShape::Circle,
                color: self.note_color,
                hold_body_color: self.hold_body_color,
                hold_cap_color: self.hold_cap_color,
            },
            1 => NoteStyle {
                shape: NoteShape::Rectangle {
                    width: 0.8,
                    height: 0.25,
                },
                color: self.note_color,
                hold_body_color: self.hold_body_color,
                hold_cap_color: self.hold_cap_color,
            },
            2 => NoteStyle {
                shape: NoteShape::Arrow {
                    width: 0.6,
                    height: 0.4,
                },
                color: self.note_color,
                hold_body_color: self.hold_body_color,
                hold_cap_color: self.hold_cap_color,
            },
            _ => NoteStyle::default(),
        }
    }

    fn load_image(&mut self, path: PathBuf) {
        let uri = format!("file://{}", path.to_string_lossy().replace('\\', "/"));
        let image_source = ImageSource::Uri(uri.into());
        let image = egui::Image::new(image_source);

        let style = NoteStyle {
            shape: NoteShape::Image(image),
            color: self.note_color,
            hold_body_color: self.hold_body_color,
            hold_cap_color: self.hold_cap_color,
        };
        self.note_style_idx = 3;
        self.player.set_note_style(style);
        self.last_error = None;
    }

    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left side controls
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Position:");
                    if ui
                        .add(
                            egui::Slider::new(&mut self.playback_time, 0.0..=self.map_duration)
                                .suffix(" ms"),
                        )
                        .changed()
                    {
                        self.player.set_current_time(self.playback_time);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Column Width:");
                    let mut changed = false;
                    if ui
                        .add(egui::Slider::new(&mut self.column_width, 50.0..=200.0).suffix(" px"))
                        .changed()
                    {
                        changed = true;
                    }

                    if changed {
                        self.reload_player_with_reset(false);
                        // Get new size and resize window
                        let mut size = self.player.get_required_size();
                        size[1] += 100.0; // Add space for bottom controls
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                                size[0], size[1],
                            )));
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Note Size:");
                    if ui
                        .add(egui::Slider::new(&mut self.note_size, 50.0..=200.0).suffix(" px"))
                        .changed()
                    {
                        self.reload_player_with_reset(false);
                    }
                });
            });

            // Center controls
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.using_ln, "LN Map").clicked() && !self.using_ln {
                        self.using_ln = true;
                        self.reload_player();
                    }
                    if ui.selectable_label(!self.using_ln, "Normal Map").clicked() && self.using_ln
                    {
                        self.using_ln = false;
                        self.reload_player();
                    }
                });

                // Show error message if any
                if let Some(error) = &self.last_error {
                    ui.colored_label(Color32::RED, error);
                }
            });

            // Right side controls
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Playback Speed:");
                    if ui
                        .add(egui::Slider::new(&mut self.playback_speed, 0.5..=2.0).suffix("x"))
                        .changed()
                    {
                        self.player.set_speed(self.playback_speed);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Scroll Speed:");
                    if ui
                        .add(
                            egui::Slider::new(&mut self.scroll_speed, 500.0..=2000.0).suffix(" ms"),
                        )
                        .changed()
                    {
                        self.player.set_scroll_time(self.scroll_speed);
                    }
                });
            });
        });

        // Bottom controls for styles
        ui.horizontal(|ui| {
            ui.label("Note Style:");
            if ui.button("Circle").clicked() {
                self.note_style_idx = 0;
                self.player.set_note_style(self.get_note_style(0));
            }
            if ui.button("Rectangle").clicked() {
                self.note_style_idx = 1;
                self.player.set_note_style(self.get_note_style(1));
            }
            if ui.button("Image").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    self.load_image(path);
                }
            }
        });

        // Colors in a collapsing section
        ui.collapsing("Colors", |ui| {
            let mut changed = false;

            ui.horizontal(|ui| {
                ui.label("Note Color:");
                if ui.color_edit_button_srgba(&mut self.note_color).changed() {
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hold Body Color:");
                if ui
                    .color_edit_button_srgba(&mut self.hold_body_color)
                    .changed()
                {
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hold Cap Color:");
                if ui
                    .color_edit_button_srgba(&mut self.hold_cap_color)
                    .changed()
                {
                    changed = true;
                }
            });

            if changed {
                self.player
                    .set_note_style(self.get_note_style(self.note_style_idx));
            }
        });
    }
}

impl eframe::App for ManiaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Central panel for gameplay
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.player.render(ui);

                // Update time if not dragging the slider
                if !ui.input(|i| i.pointer.primary_down()) {
                    self.playback_time = self.player.current_time();
                }

                ctx.request_repaint();
            });

        // Bottom panel for controls
        egui::TopBottomPanel::bottom("controls")
            .resizable(false)
            .min_height(100.0)
            .show(ctx, |ui| {
                self.draw_controls(ui);
            });
    }
}

fn main() {
    let maps_ln = include_bytes!("../assets/ln.osu");
    let maps_normal = include_bytes!("../assets/maps.osu");
    let beatmap_ln = Beatmap::from_bytes(maps_ln).expect("Failed to load LN beatmap");
    let beatmap_normal = Beatmap::from_bytes(maps_normal).expect("Failed to load normal beatmap");

    let column_width = 100.0;
    let note_size = 100.0;
    let height = 800.0;

    let app = ManiaApp::new(beatmap_ln, beatmap_normal, column_width, note_size, height)
        .expect("Unsupported game mode");
    let mut size = app.player.get_required_size();
    size[1] += 100.0; // Add space for bottom controls

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "osu!mania Player",
        options,
        Box::new(|cc| {
            // Install image loaders
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
    .expect("Failed to start application");
}
