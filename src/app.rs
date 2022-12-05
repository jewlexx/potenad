use std::{
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EditorState {
    path: Option<PathBuf>,

    // Do not include entire file contents in state saving
    #[serde(skip)]
    contents: String,

    #[serde(skip)]
    channel: (Sender<PathBuf>, Receiver<PathBuf>),
}

impl EditorState {
    pub fn save(&self) -> std::io::Result<()> {
        let config_dir = dirs::config_dir()
            .unwrap()
            .join("potenad")
            .join("config.toml");

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct EditorApp {
    state: EditorState,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            path: Default::default(),
            contents: Default::default(),
            channel: std::sync::mpsc::channel(),
        }
    }
}

impl EditorApp {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = ctx.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn open_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        use std::{fs::File, io::Read};

        let path = path.as_ref();
        let mut file = File::open(path)?;

        file.read_to_string(&mut self.contents)?;

        self.path = Some(path.to_path_buf());

        Ok(())
    }
}

impl eframe::App for EditorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }

    // TODO: Add config to be able to change this and other options
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        debug!("Updating...");

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // TODO: Save final directory
                        let file = rfd::FileDialog::new().pick_file();

                        if let Some(path) = file {
                            self.open_file(path).unwrap();
                        }
                    }

                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_height(100.0);
            ui.set_max_height(100.0);
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.contents),
            );
        });
    }
}
