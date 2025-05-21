#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod utils;

use crate::config::Config;
use crate::utils::{generate_password, generate_password_to_ctx};
use eframe::egui;
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Button, Heading, Monospace, Name, Small};
use eframe::egui::{FontId, Vec2};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400., 320.]),
        ..Default::default()
    };
    eframe::run_native(
        "K Pass Gen",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            cc.egui_ctx.style_mut(|style| {
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
        let cfg = Config::new();
        let passwd = generate_password(cfg.len, &cfg.charset, cfg.use_capitals);
        Self {
            config: cfg,
            password: passwd,
        }
    }
}

impl Default for KPassGenApp {
    fn default() -> Self {
        Self {
            config: Config::new(),
            password: String::new(),
        }
    }
}

impl eframe::App for KPassGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    generate_password_to_ctx(self);
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
                ui.add(egui::Slider::new(&mut self.config.len, 2..=100));
            });
        });
    }
}
