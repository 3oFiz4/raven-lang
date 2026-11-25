mod audio;
mod phonology;

use audio::AudioPlayer;
use eframe::egui;
use phonology::{convert_type_to_ipa, PhonemeEntry, CONSONANT_INVENTORY, VOWEL_INVENTORY};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([860.0, 580.0])
            .with_min_inner_size([620.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Raven_Lang — Type Keybind & Phonetics",
        options,
        Box::new(|cc| {
            configure_ipa_fonts(&cc.egui_ctx);
            Ok(Box::new(RavenApp::new()))
        }),
    )
}

/// Loads Windows system fonts with full IPA Unicode coverage as fallbacks.
fn configure_ipa_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let candidates = [
        ("lucida_unicode", r"C:\Windows\Fonts\l_10646.ttf"),
        ("segoe_ui", r"C:\Windows\Fonts\segoeui.ttf"),
    ];

    for (name, path) in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert(name.to_owned(), egui::FontData::from_owned(bytes));

            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                family.insert(0, name.to_owned());
            }
            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                family.push(name.to_owned());
            }
        }
    }

    ctx.set_fonts(fonts);
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct HoveredExample {
    id: egui::Id,
    anchor_pos: egui::Pos2,
    entry: &'static PhonemeEntry,
}

struct RavenApp {
    input: String,
    show_reference: bool,
    audio: AudioPlayer,
    active_example: Option<HoveredExample>,
}

impl RavenApp {
    fn new() -> Self {
        Self {
            input: String::from("ng*i |eK ch*i|e"),
            show_reference: true,
            audio: AudioPlayer::new(),
            active_example: None,
        }
    }
}

/// Compact circular play icon button.
fn play_icon_button(ui: &mut egui::Ui, size: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let center = rect.center();
        let radius = size * 0.45;
        let fill = if response.hovered() {
            ui.visuals().selection.bg_fill
        } else {
            visuals.bg_fill
        };
        ui.painter()
            .circle(center, radius, fill, visuals.bg_stroke);

        let r = radius * 0.48;
        let p1 = egui::pos2(center.x - r * 0.55, center.y - r * 0.85);
        let p2 = egui::pos2(center.x - r * 0.55, center.y + r * 0.85);
        let p3 = egui::pos2(center.x + r * 0.95, center.y);
        ui.painter().add(egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            visuals.text_color(),
            egui::Stroke::NONE,
        ));
    }
    response
}

/// Single `(e)` circle icon; turns blue when hovered or active.
fn example_circle_button(ui: &mut egui::Ui, size: f32, is_active: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    let blue_active = response.hovered() || is_active;
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let center = rect.center();
        let radius = size * 0.45;
        let (fill, stroke, text_color) = if blue_active {
            (
                egui::Color32::from_rgb(35, 110, 220),
                egui::Stroke::new(1.3, egui::Color32::from_rgb(130, 195, 255)),
                egui::Color32::WHITE,
            )
        } else {
            (
                ui.visuals().widgets.inactive.bg_fill,
                visuals.bg_stroke,
                visuals.text_color(),
            )
        };
        ui.painter().circle(center, radius, fill, stroke);
        ui.painter().text(
            center - egui::vec2(0.0, 0.5),
            egui::Align2::CENTER_CENTER,
            "e",
            egui::FontId::proportional(size * 0.62),
            text_color,
        );
    }
    response
}

fn render_phoneme_table(
    ui: &mut egui::Ui,
    id: &str,
    entries: &'static [PhonemeEntry],
    input: &mut String,
    audio: &AudioPlayer,
    active_example: &mut Option<HoveredExample>,
    next_hovered: &mut Option<HoveredExample>,
) {
    egui::Grid::new(id)
        .striped(true)
        .spacing([6.0, 3.0])
        .num_columns(4)
        .show(ui, |ui| {
            for entry in entries {
                if let Some(kb) = entry.keybind {
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(kb).monospace().size(12.0))
                                .min_size(egui::vec2(34.0, 18.0)),
                        )
                        .on_hover_text(format!("Insert \"{kb}\""))
                        .clicked()
                    {
                        input.push_str(kb);
                    }
                } else {
                    ui.label(egui::RichText::new("—").weak().monospace());
                }

                ui.label(egui::RichText::new(format!("/{}/", entry.ipa)).size(14.0));

                if play_icon_button(ui, 18.0)
                    .on_hover_text(format!("Play /{}/", entry.ipa))
                    .clicked()
                {
                    audio.play(entry.sound_file);
                }

                let row_id = ui.make_persistent_id((id, entry.ipa));
                let is_active = active_example
                    .as_ref()
                    .is_some_and(|active| active.id == row_id);
                let resp = example_circle_button(ui, 18.0, is_active);
                if resp.hovered() {
                    *next_hovered = Some(HoveredExample {
                        id: row_id,
                        anchor_pos: resp.rect.left_bottom() + egui::vec2(0.0, 2.0),
                        entry,
                    });
                }

                ui.end_row();
            }
        });
}

impl eframe::App for RavenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let output = convert_type_to_ipa(&self.input);
        let mut next_hovered: Option<HoveredExample> = None;

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.heading("Raven_Lang ASCII Type -> IPA");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.toggle_value(&mut self.show_reference, "Phonemes & Keybinds");
                });
            });
            ui.add_space(4.0);
        });

        if self.show_reference {
            egui::SidePanel::right("reference_panel")
                .min_width(185.0)
                .default_width(205.0)
                .show(ctx, |ui| {
                    ui.heading("Phonemes");
                    ui.label(
                        egui::RichText::new("keybind • ▶ sound • (e) examples")
                            .small()
                            .weak(),
                    );
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.strong(format!("Vowels ({})", VOWEL_INVENTORY.len()));
                        ui.add_space(2.0);
                        render_phoneme_table(
                            ui,
                            "vowel_grid",
                            VOWEL_INVENTORY,
                            &mut self.input,
                            &self.audio,
                            &mut self.active_example,
                            &mut next_hovered,
                        );

                        ui.add_space(10.0);
                        ui.strong(format!("Consonants ({})", CONSONANT_INVENTORY.len()));
                        ui.add_space(2.0);
                        render_phoneme_table(
                            ui,
                            "consonant_grid",
                            CONSONANT_INVENTORY,
                            &mut self.input,
                            &self.audio,
                            &mut self.active_example,
                            &mut next_hovered,
                        );
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.strong("Input (Type Keybind Text)");
            ui.add_space(4.0);
            ui.add(
                egui::TextEdit::multiline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .desired_rows(6)
                    .font(egui::TextStyle::Monospace)
                    .hint_text("Type ASCII keybinds here, e.g. ng*i |eK ch*i|e"),
            );

            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.strong("Output (IPA Characters)");
                if ui.button("Copy Output").clicked() {
                    ctx.copy_text(output.clone());
                }
                if ui.button("Clear").clicked() {
                    self.input.clear();
                }
            });
            ui.add_space(4.0);

            let mut output_display = output;
            ui.add(
                egui::TextEdit::multiline(&mut output_display)
                    .desired_width(f32::INFINITY)
                    .desired_rows(6)
                    .font(egui::FontId::proportional(22.0))
                    .interactive(false),
            );
        });

        // Interactive hover tooltip for `(e)`: stays open while `(e)` or the popup is hovered.
        let display_target = next_hovered.or(self.active_example);
        let mut popup_hovered = false;
        if let Some(target) = display_target {
            let screen_rect = ctx.screen_rect();
            let pos = egui::pos2(
                target.anchor_pos.x.min(screen_rect.right() - 320.0).max(8.0),
                target.anchor_pos.y.min(screen_rect.bottom() - 60.0).max(8.0),
            );

            let area_resp = egui::Area::new(egui::Id::new("example_words_popup"))
                .order(egui::Order::Tooltip)
                .fixed_pos(pos)
                .show(ctx, |ui| {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 2.0;
                            let len = target.entry.examples.len();
                            for (i, ex) in target.entry.examples.iter().enumerate() {
                                let word_btn = ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new(ex.word)
                                                .color(egui::Color32::from_rgb(90, 170, 255)),
                                        )
                                        .frame(false),
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text(format!("[{}] Click to play", ex.lang));

                                if word_btn.clicked() {
                                    self.audio.play(ex.sound_file);
                                }

                                if i + 1 < len {
                                    ui.label(", ");
                                }
                            }
                        });
                    });
                });

            let pointer_in_popup = ctx.input(|i| {
                i.pointer
                    .hover_pos()
                    .is_some_and(|p| area_resp.response.rect.expand(6.0).contains(p))
            });
            popup_hovered = pointer_in_popup || area_resp.response.hovered();
        }

        self.active_example = if next_hovered.is_some() {
            next_hovered
        } else if popup_hovered {
            self.active_example
        } else {
            None
        };
    }
}
