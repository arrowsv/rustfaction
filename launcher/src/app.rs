use common::config::Config;
use eframe::{App, egui::{self, Context}};
use anyhow::Result;

const BACKGROUND_IMAGE: egui::ImageSource = egui::include_image!("../../assets/background.png");
const INTER_FONT: &'static [u8] = include_bytes!("../../assets/inter-medium.ttf");

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_inner_size(egui::vec2(576.0, 324.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Faction Launcher",
        options,
        Box::new(|creation_context| {
            creation_context
                .egui_ctx.set_theme(egui::Theme::Dark);
            
            let app = Launcher::new(creation_context.egui_ctx.clone())
                .expect("Failed to create launcher app");
            
            Ok(Box::new(app))
        }),
    )
}

#[derive(Default)]
struct Launcher {
    options_open: bool,
    config: Config,
}

impl Launcher {
    pub fn new(ctx: Context) -> Result<Self> {
        egui_extras::install_image_loaders(&ctx);

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "inter".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(INTER_FONT)),
        );
        fonts.families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "inter".to_owned());
        ctx.set_fonts(fonts);

        Ok(Self::default())
    }
}

impl App for Launcher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Image::new(BACKGROUND_IMAGE)
                .paint_at(ui, ctx.viewport_rect());

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        if let Err(e) = crate::launch_game() {
                            common::utils::show_error_message(&format!("Failed to launch game: {}", e));
                        }
                    }
        
                    if ui.button("Options").clicked() {
                        self.config = Config::get();
                        self.options_open = true;
                    }
                });
            });
        });

        if self.options_open {
            let modal = egui::Modal::new(egui::Id::new("Options")).show(ctx, |ui| {
                ui.label("Game directory");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.config.game_directory);
                    if ui.button("Browse").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.config.game_directory = path.to_string_lossy().to_string();
                        }
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("FPS limit");
                    ui.add(egui::DragValue::new(&mut self.config.fps_limit));
                });

                ui.checkbox(&mut self.config.fast_start, "Fast start")
                    .on_hover_text("Removes the game intro sequence");

                ui.checkbox(&mut self.config.use_overrides, "Use overrides")
                    .on_hover_text("Enable loading of packfiles from the 'overrides' folder");

                ui.checkbox(&mut self.config.keep_launcher_open, "Keep launcher open")
                    .on_hover_text("Keep the launcher open after launching the game");

                ui.checkbox(&mut self.config.show_console, "Show console")
                    .on_hover_text("Attaches a terminal to the game which prints log messages");

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        Config::set(self.config.clone());
                        self.options_open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.options_open = false;
                    }
                });
            });

            if modal.should_close() {
                self.options_open = false;
            }
        }
    }
}