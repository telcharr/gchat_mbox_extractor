use eframe::{CreationContext, egui, NativeOptions};
use std::path::PathBuf;
use std::time::Instant;
use std::sync::mpsc::{channel, Receiver, Sender};
use rfd::FileDialog;
use std::thread;

use super::components::{Action, render_ui};
use super::animations::update_animations;
use super::processing::process_mbox;

/// Represents the main application state
pub struct MboxExtractorApp {
    pub mbox_path: Option<PathBuf>,
    pub export_attachments: bool,
    pub output_path: Option<PathBuf>,
    pub processing: bool,
    pub result: String,
    pub progress: Option<f32>,
    pub animated_progress: f32,
    pub button_animations: [f32; 3],
    pub button_hover_states: [bool; 3],
    pub result_animation: f32,
    pub processing_start_time: Option<Instant>,
    pub result_start_time: Option<Instant>,
    progress_rx: Option<Receiver<f32>>,
    result_rx: Option<Receiver<String>>,
    processing_complete: bool,
    file_dialog_rx: Receiver<Option<PathBuf>>,
    file_dialog_tx: Sender<Option<PathBuf>>,
    folder_dialog_rx: Receiver<Option<PathBuf>>,
    folder_dialog_tx: Sender<Option<PathBuf>>,
}

impl MboxExtractorApp {
    fn new() -> Self {
        let (file_dialog_tx, file_dialog_rx) = channel();
        let (folder_dialog_tx, folder_dialog_rx) = channel();

        Self {
            mbox_path: None,
            export_attachments: false,
            output_path: None,
            processing: false,
            result: String::new(),
            progress: None,
            animated_progress: 0.0,
            button_animations: [0.0; 3],
            button_hover_states: [false; 3],
            result_animation: 0.0,
            processing_start_time: None,
            result_start_time: None,
            progress_rx: None,
            result_rx: None,
            processing_complete: false,
            file_dialog_rx,
            file_dialog_tx,
            folder_dialog_rx,
            folder_dialog_tx,
        }
    }

    /// This is called by the eframe runtime on each frame.
    fn update(&mut self, action: Action) {
        match action {
            Action::OpenMboxFileDialog => self.open_file_dialog(),
            Action::OpenOutputFolderDialog => self.open_folder_dialog(),
            Action::ToggleExportAttachments => self.export_attachments = !self.export_attachments,
            Action::StartProcessing => self.start_processing(),
            Action::UpdateProgress(progress) => self.progress = Some(progress),
            Action::FinishProcessing => self.finish_processing(),
            Action::SetButtonHoverState(index, hovered) => {
                self.button_hover_states[index] = hovered;
            },
        }
    }

    /// Opens the file dialogue window
    fn open_file_dialog(&mut self) {
        let tx: Sender<Option<PathBuf>> = self.file_dialog_tx.clone();
        thread::spawn(move || {
            let result: Option<PathBuf> = FileDialog::new()
                .add_filter("MBOX", &["mbox"])
                .pick_file();
            tx.send(result).unwrap();
        });
    }

    /// Opens the folder dialogue window
    fn open_folder_dialog(&mut self) {
        let tx: Sender<Option<PathBuf>> = self.folder_dialog_tx.clone();
        thread::spawn(move || {
            let result: Option<PathBuf> = FileDialog::new().pick_folder();
            tx.send(result).unwrap();
        });
    }

    /// Initializes MBOX processing
    fn start_processing(&mut self) {
        self.processing = true;
        self.result = String::new();
        self.progress = Some(0.0);
        self.animated_progress = 0.0;
        self.processing_start_time = Some(Instant::now());
        self.processing_complete = false;
        self.process_mbox_if_ready();
    }

    /// Processes the MBOX file if ready
    fn process_mbox_if_ready(&mut self) {
        if self.processing {
            let mbox_path: Option<PathBuf> = self.mbox_path.clone();
            let output_path: Option<PathBuf> = self.output_path.clone();
            let export_attachments: bool = self.export_attachments;

            if let (Some(mbox_path), Some(output_path)) = (mbox_path, output_path) {
                let (progress_tx, progress_rx) = channel();
                let (result_tx, result_rx) = channel();

                self.progress_rx = Some(progress_rx);
                self.result_rx = Some(result_rx);

                thread::spawn(move || {
                    process_mbox(&mbox_path, &output_path, export_attachments, progress_tx, result_tx);
                });
            }
        }
    }

    /// Called when MBOX processing is finished to reset the application state.
    fn finish_processing(&mut self) {
        self.processing = false;
        self.progress = None;
        self.animated_progress = 0.0;
        self.processing_start_time = None;
        self.result_start_time = Some(Instant::now());
        self.result_animation = 0.0;
        self.progress_rx = None;
        self.result_rx = None;
        self.processing_complete = false;
    }

    /// Called to check for and update file dialogue results
    fn check_file_dialog_results(&mut self) {
        if let Ok(result) = self.file_dialog_rx.try_recv() {
            self.mbox_path = result;
        }
        if let Ok(result) = self.folder_dialog_rx.try_recv() {
            self.output_path = result;
        }
    }
}

impl eframe::App for MboxExtractorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let actions: Vec<Action> = render_ui(self, ctx);
        for action in actions {
            self.update(action);
        }

        self.check_file_dialog_results();

        if let Some(rx) = &self.progress_rx {
            if let Ok(progress) = rx.try_recv() {
                self.progress = Some(progress);
                if progress >= 1.0 {
                    self.processing_complete = true;
                }
            }
        }

        if let Some(rx) = &self.result_rx {
            if let Ok(result) = rx.try_recv() {
                self.result = result;
            }
        }

        update_animations(self, ctx);

        if self.processing_complete && self.animated_progress > 0.99 {
            self.animated_progress = 1.0;
            self.finish_processing();
        }

        ctx.request_repaint();
    }
}

/// Sets up the eframe native options, initializes the application state, and starts the main event loop.
pub fn run_ui() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 450.0])
            .with_min_inner_size([400.0, 450.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "GChat MBOX Extractor",
        options,
        Box::new(|cc: &CreationContext| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MboxExtractorApp::new()))
        }),
    )
}