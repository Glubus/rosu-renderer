use egui::{self, pos2, Color32, Rect, Vec2};
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};

#[derive(Clone)]
pub enum NoteShape {
    Circle,
    Rectangle { width: f32, height: f32 },
    Arrow { width: f32, height: f32 },
    Image(egui::Image<'static>),
}

pub struct NoteStyle {
    pub shape: NoteShape,
    pub color: Color32,
    pub hold_body_color: Color32,
    pub hold_cap_color: Color32,
}

impl Default for NoteStyle {
    fn default() -> Self {
        Self {
            shape: NoteShape::Rectangle {
                width: 0.8,
                height: 0.25,
            }, // Rectangle par défaut
            color: Color32::from_rgb(0, 174, 255),
            hold_body_color: Color32::from_rgb(200, 200, 200),
            hold_cap_color: Color32::from_rgb(0, 174, 255),
        }
    }
}

pub struct ManiaRenderer {
    column_width: f32,
    note_size: f32,
    speed: f64,
    height: f32,
    note_style: NoteStyle,
}

impl ManiaRenderer {
    pub fn with_sizes(column_width: f32, note_size: f32, height: f32) -> Self {
        Self {
            column_width,
            note_size,
            speed: 1.0,
            height,
            note_style: NoteStyle::default(),
        }
    }

    pub fn set_note_style(&mut self, style: NoteStyle) {
        self.note_style = style;
    }

    fn draw_note(&self, ui: &mut egui::Ui, x_pos: f32, y_pos: f32) {
        let center_x = x_pos + self.column_width / 2.0;

        match &self.note_style.shape {
            NoteShape::Circle => {
                let circle_radius = self.note_size / 2.0;
                ui.painter().circle_filled(
                    pos2(center_x, y_pos),
                    circle_radius,
                    self.note_style.color,
                );
            }
            NoteShape::Rectangle { width, height } => {
                let note_width = self.note_size * width;
                let note_height = self.note_size * height;
                let rect = Rect::from_center_size(
                    pos2(center_x, y_pos),
                    Vec2::new(note_width, note_height),
                );
                ui.painter().rect_filled(rect, 0.0, self.note_style.color);
            }
            NoteShape::Arrow { width, height } => {
                let note_width = self.note_size * width;
                let note_height = self.note_size * height;
                let points = vec![
                    pos2(center_x, y_pos - note_height / 2.0), // Pointe
                    pos2(center_x + note_width / 2.0, y_pos + note_height / 2.0), // Droite
                    pos2(center_x - note_width / 2.0, y_pos + note_height / 2.0), // Gauche
                ];
                ui.painter().add(egui::Shape::convex_polygon(
                    points,
                    self.note_style.color,
                    egui::Stroke::NONE,
                ));
            }
            NoteShape::Image(image) => {
                image.paint_at(
                    ui,
                    Rect::from_min_size(
                        pos2(
                            center_x - self.note_size / 2.0,
                            y_pos - self.note_size / 2.0,
                        ),
                        Vec2::new(self.note_size, self.note_size),
                    ),
                );
            }
        }
    }

    fn render_hold(
        &self,
        ui: &mut egui::Ui,
        x_pos: f32,
        start_y: f32,
        end_y: f32,
        judgment_line_y: f32,
    ) {
        let note_width = self.note_size * 0.8;
        let x_center = x_pos + (self.column_width - note_width) / 2.0;

        let y_start = start_y.min(end_y);
        let y_end = (start_y.max(end_y)).min(judgment_line_y);
        let visible_height = (y_end - y_start).abs();

        // Hold body
        ui.painter().rect_filled(
            Rect::from_min_size(
                pos2(x_center, y_start),
                Vec2::new(note_width, visible_height),
            ),
            0.0,
            self.note_style.hold_body_color,
        );

        // Hold end cap
        let cap_height = note_width * 0.3;
        if end_y <= judgment_line_y {
            ui.painter().rect_filled(
                Rect::from_min_size(pos2(x_center, end_y), Vec2::new(note_width, cap_height)),
                0.0,
                self.note_style.hold_cap_color,
            );
        }
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn required_width(&self, keycount: usize) -> f32 {
        self.column_width * keycount as f32
    }

    pub fn required_height(&self) -> f32 {
        self.height
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        hit_objects: &[HitObject],
        current_time: f64,
        scroll_time_ms: f32,
        speed: f64,
        keycount: usize,
    ) {
        self.render_at(ui, hit_objects, current_time, scroll_time_ms, speed, keycount, pos2(0.0, 0.0))
    }

    pub fn render_at(
        &mut self,
        ui: &mut egui::Ui,
        hit_objects: &[HitObject],
        current_time: f64,
        scroll_time_ms: f32,
        speed: f64,
        keycount: usize,
        position: egui::Pos2,
    ) {
        self.speed = speed;

        let total_width = self.required_width(keycount);
        let total_height = self.required_height();

        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            let rect = Rect::from_min_size(position, Vec2::new(total_width, total_height));

            ui.set_min_size(Vec2::new(total_width, total_height));
            ui.set_max_size(Vec2::new(total_width, total_height));

            let play_area = rect;
            let clip_rect = ui.clip_rect().intersect(play_area);
            ui.set_clip_rect(clip_rect);

            // Draw columns
            for i in 0..keycount {
                let column_rect = Rect::from_min_size(
                    pos2(position.x + i as f32 * self.column_width, position.y),
                    Vec2::new(self.column_width, total_height),
                );
                ui.painter()
                    .rect_filled(column_rect, 0.0, egui::Color32::from_gray(20));
            }

            let judgment_line_y = position.y + total_height - 100.0;
            ui.painter().line_segment(
                [
                    pos2(position.x, judgment_line_y),
                    pos2(position.x + total_width, judgment_line_y),
                ],
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );

            if hit_objects.last().is_some() {
                // Draw hold notes first
                for hit_object in hit_objects
                    .iter()
                    .filter(|h| matches!(h.kind, HitObjectKind::Hold(_)))
                {
                    if let HitObjectKind::Hold(h) = &hit_object.kind {
                        let column = (h.pos_x / 512.0 * keycount as f32) as usize % keycount;
                        let x_pos = position.x + column as f32 * self.column_width;

                        let note_time = hit_object.start_time / speed + scroll_time_ms as f64;
                        let end_time =
                            (hit_object.start_time + h.duration) / speed + scroll_time_ms as f64;

                        let time_diff = note_time - current_time;
                        let end_time_diff = end_time - current_time;

                        let y_pos =
                            judgment_line_y - (time_diff as f32 / scroll_time_ms) * total_height;
                        let end_y_pos = judgment_line_y
                            - (end_time_diff as f32 / scroll_time_ms) * total_height;

                        if end_y_pos <= judgment_line_y {
                            self.render_hold(ui, x_pos, y_pos, end_y_pos, judgment_line_y);
                        }
                    }
                }

                // Then draw regular notes
                for hit_object in hit_objects {
                    let note_time = hit_object.start_time / speed + scroll_time_ms as f64;
                    let time_diff = note_time - current_time;
                    let y_pos =
                        judgment_line_y - (time_diff as f32 / scroll_time_ms) * total_height;

                    if y_pos <= judgment_line_y {
                        let x_pos = match &hit_object.kind {
                            HitObjectKind::Circle(h) => {
                                let column =
                                    (h.pos.x / 512.0 * keycount as f32) as usize % keycount;
                                position.x + column as f32 * self.column_width
                            }
                            HitObjectKind::Hold(h) => {
                                let column =
                                    (h.pos_x / 512.0 * keycount as f32) as usize % keycount;
                                position.x + column as f32 * self.column_width
                            }
                            _ => continue,
                        };

                        if y_pos >= 0.0 {
                            self.draw_note(ui, x_pos, y_pos);
                        }
                    }
                }
            }
        });
    }
}
