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
    pub fn load() -> std::io::Result<Self> {
        let config_path = Self::config_path();

        let bytes = std::fs::read(config_path)?;

        Ok(toml::from_slice(&bytes)?)
    }

    pub fn save(&self) -> std::io::Result<()> {
        let config_path = Self::config_path();

        let bytes = toml::to_vec(self)?;

        std::fs::create_dir_all(&config_path)?;

        std::fs::write(&config_path, &bytes)?;

        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join("potenad")
            .join("config.toml")
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
    pub fn new() -> Self {
        Self {
            state: EditorState::load().unwrap_or_default(),
        }
    }

    pub fn open_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        use std::{fs::File, io::Read};

        let path = path.as_ref();
        let mut file = File::open(path)?;

        file.read_to_string(&mut self.state.contents)?;

        self.state.path = Some(path.to_path_buf());

        Ok(())
    }

    pub fn save_state(&self) {
        self.state.save()
    }
}

impl iced::Application for EditorApp {
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
