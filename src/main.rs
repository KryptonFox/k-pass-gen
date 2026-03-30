#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod generator;

use crate::config::{Charset, Config};
use crate::generator::generate_password;
use eframe::egui;
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Button, Heading, Monospace, Name, Small};
use eframe::egui::{FontId, Vec2};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500., 420.]),
        ..Default::default()
    };
    eframe::run_native(
        "K Pass Gen",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            cc.egui_ctx.global_style_mut(|style| {
                style.spacing.slider_width = 167.0;
                style.spacing.button_padding = Vec2::from((10., 4.));
                style.spacing.item_spacing = Vec2::from((10., 10.));
                style.text_styles = [
                    (Heading, FontId::new(28.0, Proportional)),
                    (Name("Heading2".into()), FontId::new(25.0, Proportional)),
                    (Name("Context".into()), FontId::new(23.0, Proportional)),
                    (Body, FontId::new(19.5, Proportional)),
                    (Monospace, FontId::new(14.0, Proportional)),
                    (Button, FontId::new(20.0, Proportional)),
                    (Small, FontId::new(10.0, Proportional)),
                ]
                .into()
            });
            Ok(Box::new(KPassGenApp::new()))
        }),
    )
}

pub struct KPassGenApp {
    pub config: Config,
    pub password: String,
}

impl KPassGenApp {
    pub fn new() -> Self {
        let cfg = Config::load();
        let passwd = generate_password(&cfg);
        Self {
            config: cfg,
            password: passwd,
        }
    }
}

impl eframe::App for KPassGenApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| ui.heading("K Password Generator"));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.centered_and_justified(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.password))
                })
            });
            ui.horizontal(|ui| {
                let generate_image = egui::include_image!("../res/generate.png");
                if ui
                    .add(egui::Button::image_and_text(generate_image, "Generate"))
                    .clicked()
                {
                    self.password = generate_password(&self.config);
                }
                let copy_image = egui::include_image!("../res/copy.png");
                if ui
                    .add(egui::Button::image_and_text(copy_image, "Copy"))
                    .clicked()
                {
                    ui.ctx().copy_text(self.password.clone());
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Length: ");
                ui.add(egui::Slider::new(&mut self.config.len, 1..=128));
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("Config", |ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::Checkbox::without_text(
                            &mut self.config.letters.enabled,
                        ));
                        ui.label("Letters: ");
                        ui.centered_and_justified(|ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.config.letters.chars))
                        })
                    });

                    ui.horizontal(|ui| {
                        ui.label("Use capitals: ");
                        ui.add(egui::Checkbox::without_text(
                            &mut self.config.letters.use_capitals,
                        ));
                    });

                    let mut to_delete = vec![];
                    for (index, charset) in self.config.charsets.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("X")).clicked() {
                                to_delete.push(index);
                            }
                            ui.add(egui::Checkbox::without_text(&mut charset.enabled));

                            if charset.name_editing {
                                ui.add(
                                    egui::TextEdit::singleline(&mut charset.name)
                                        .desired_width(0.0)
                                        .clip_text(false),
                                );
                            } else {
                                ui.label(&charset.name);
                            }
                            if ui.button("✏").clicked() {
                                charset.name_editing = !charset.name_editing;
                            }
                            ui.label(": ");
                            ui.centered_and_justified(|ui| {
                                ui.add(egui::TextEdit::singleline(&mut charset.chars))
                            });
                        });
                    }
                    ui.horizontal(|ui| {
                        if ui.button("+").clicked() {
                            self.config.charsets.push(Charset::new(format!(
                                "Charset ({})",
                                self.config.charsets.len() + 1
                            )))
                        }
                        if ui.button("Reset").clicked() {
                            self.config = Config::default();
                        }
                    });

                    for i in to_delete {
                        self.config.charsets.remove(i);
                    }
                });
            });

            if ui.ctx().input(|i| i.viewport().close_requested()) {
                self.config.save()
            }
        });
    }
}
