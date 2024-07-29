use std::error::Error;
use eframe::{CreationContext, egui, epaint::Color32, NativeOptions};
use rfd::FileDialog;
use std::path::PathBuf;
use eframe::emath::NumExt;
use egui::{Frame, Rounding, Stroke, RichText, Vec2, Ui, InputState, Rect, ProgressBar};
use std::time::{Instant, Duration};
use egui::style::WidgetVisuals;
use crate::models::{MboxEntry, Message};
use crate::parsers::split_mbox_entries;
use crate::utils::{read_mbox_file, write_messages_to_csv, write_attachment_to_file};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

const BG_COLOR: Color32 = Color32::from_rgb(33, 37, 41);
const ACCENT_COLOR: Color32 = Color32::from_rgb(72,77,83);
const TEXT_COLOR: Color32 = Color32::from_rgb(211,211,212);
const BUTTON_COLOR: Color32 = Color32::from_rgb(52,58,64);
const HOVER_COLOR: Color32 = Color32::from_rgb(72,77,83);

pub struct MboxExtractorApp {
    mbox_path: Option<PathBuf>,
    export_attachments: bool,
    output_path: Option<PathBuf>,
    processing: bool,
    result: String,
    progress: Option<f32>,
    animated_progress: f32,
    button_animations: [f32; 3],
    result_animation: f32,
    processing_start_time: Option<Instant>,
    result_start_time: Option<Instant>,
    progress_rx: Option<Receiver<f32>>,
    result_rx: Option<Receiver<String>>,
    processing_complete: bool,
}

impl Default for MboxExtractorApp {
    fn default() -> Self {
        Self {
            mbox_path: None,
            export_attachments: false,
            output_path: None,
            processing: false,
            result: String::new(),
            progress: None,
            animated_progress: 0.0,
            button_animations: [0.0; 3],
            result_animation: 0.0,
            processing_start_time: None,
            result_start_time: None,
            progress_rx: None,
            result_rx: None,
            processing_complete: false,
        }
    }
}

impl eframe::App for MboxExtractorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame: Frame = Frame::default()
            .fill(BG_COLOR)
            .inner_margin(20.0);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui: &mut Ui| {
            ui.vertical_centered(|ui: &mut Ui| {
                ui.add_space(20.0);
                ui.heading(RichText::new("GChat MBOX Extractor").color(TEXT_COLOR).size(24.0));
                ui.add_space(30.0);

                if !self.processing {
                    self.render_file_selection(ui);
                    ui.add_space(20.0);
                    self.render_output_selection(ui);
                    ui.add_space(20.0);
                    self.render_process_button(ui);
                } else {
                    self.render_processing(ui);
                }

                ui.add_space(30.0);
                self.render_result(ui);
            });
        });

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

        self.update_animations(ctx);

        if self.processing_complete && self.animated_progress > 0.99 {
            self.animated_progress = 1.0; // Force to 100%
            self.finish_processing();
        }

        ctx.request_repaint();
    }
}

impl MboxExtractorApp {
    fn render_file_selection(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui: &mut Ui| {
            if self.animated_button(ui, "Select MBOX File", 0) {
                if let Some(path) = FileDialog::new().add_filter("MBOX", &["mbox"]).pick_file() {
                    self.mbox_path = Some(path);
                }
            }
            ui.add_space(10.0);
            if let Some(path) = &self.mbox_path {
                ui.label(RichText::new(format!("File: {}", path.file_name().unwrap_or_default().to_string_lossy())).color(TEXT_COLOR));
            }
        });
    }

    fn render_output_selection(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui: &mut Ui| {
            if self.animated_button(ui, "Select Output Folder", 1) {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.output_path = Some(path);
                }
            }
            ui.add_space(10.0);
            if let Some(path) = &self.output_path {
                ui.label(RichText::new(format!("Folder: {}", path.file_name().unwrap_or_default().to_string_lossy())).color(TEXT_COLOR));
            }
            ui.add_space(10.0);
            ui.checkbox(&mut self.export_attachments, RichText::new("Export Attachments").color(TEXT_COLOR));
        });
    }

    fn render_process_button(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui: &mut Ui| {
            if self.mbox_path.is_some() && self.output_path.is_some() {
                if self.animated_button(ui, "Process MBOX", 2) {
                    self.processing = true;
                    self.result = String::new();
                    self.progress = Some(0.0);
                    self.animated_progress = 0.0;
                    self.processing_start_time = Some(Instant::now());
                    self.processing_complete = false;
                    self.process_mbox_if_ready();
                }
            }
        });
    }

    fn render_processing(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui: &mut Ui| {
            ui.add_space(20.0);
            let progress_bar: ProgressBar = ProgressBar::new(self.animated_progress)
                .animate(true)
                .show_percentage()
                .desired_width(200.0);
            ui.add(progress_bar);
            ui.label(RichText::new("Processing...").color(TEXT_COLOR));
        });
    }

    fn render_result(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui: &mut Ui| {
            if !self.result.is_empty() {
                let color = Color32::from_rgba_unmultiplied(
                    TEXT_COLOR.r(),
                    TEXT_COLOR.g(),
                    TEXT_COLOR.b(),
                    (self.result_animation * 255.0) as u8,
                );
                ui.label(RichText::new(&self.result).color(color));
            }
        });
    }

    fn update_animations(&mut self, ctx: &egui::Context) {
        let dt: f32 = ctx.input(|i: &InputState| i.unstable_dt).min(0.1);

        // Update animated progress
        if let Some(progress) = self.progress {
            self.animated_progress += (progress - self.animated_progress) * dt * 5.0;
            self.animated_progress = self.animated_progress.clamp(0.0, 1.0);
        }

        if !self.processing && !self.result.is_empty() {
            self.result_animation += dt * 2.0;
            self.result_animation = self.result_animation.min(1.0);
        }

        if let Some(start_time) = self.result_start_time {
            if start_time.elapsed() > Duration::from_secs(5) {
                self.result_animation -= dt * 2.0;
                self.result_animation = self.result_animation.max(0.0);

                if self.result_animation == 0.0 {
                    self.result = String::new();
                    self.result_start_time = None;
                }
            }
        }
    }

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
                    let result: Result<(), Box<dyn Error>> = process_mbox(&mbox_path, &output_path, export_attachments, progress_tx);
                    match result {
                        Ok(()) => result_tx.send("Processing completed successfully.".to_string()).unwrap(),
                        Err(e) => result_tx.send(format!("Error: {}", e)).unwrap(),
                    }
                });
            }
        }
    }

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

    fn animated_button(&mut self, ui: &mut Ui, text: &str, index: usize) -> bool {
        let desired_size: Vec2 = Vec2::new(200.0, 40.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if response.hovered() {
            self.button_animations[index] += ui.ctx().input(|i: &InputState| i.unstable_dt).at_most(8.0);
        } else {
            self.button_animations[index] -= ui.ctx().input(|i: &InputState| i.unstable_dt).at_most(2.5);
        }
        self.button_animations[index] = self.button_animations[index].clamp(0.0, 1.0);

        let lerp: fn(f32, f32, f32) -> f32  = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let fill_color = Color32::from_rgb(
            lerp(BUTTON_COLOR.r() as f32, HOVER_COLOR.r() as f32, self.button_animations[index]) as u8,
            lerp(BUTTON_COLOR.g() as f32, HOVER_COLOR.g() as f32, self.button_animations[index]) as u8,
            lerp(BUTTON_COLOR.b() as f32, HOVER_COLOR.b() as f32, self.button_animations[index]) as u8,
        );

        let visuals: &WidgetVisuals = ui.style().noninteractive();
        let rect: Rect = rect.expand(visuals.expansion);
        ui.painter().rect(
            rect,
            Rounding::same(8.0),
            fill_color,
            Stroke::new(1.0, ACCENT_COLOR),
        );

        let text_color: Color32 = Color32::from_rgba_premultiplied(
            TEXT_COLOR.r(),
            TEXT_COLOR.g(),
            TEXT_COLOR.b(),
            lerp(200.0, 255.0, self.button_animations[index]) as u8,
        );

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(16.0),
            text_color,
        );

        response.clicked()
    }
}

fn process_mbox(
    mbox_path: &PathBuf,
    output_path: &PathBuf,
    export_attachments: bool,
    progress_tx: Sender<f32>,
) -> Result<(), Box<dyn Error>> {
    let mbox_content: String = read_mbox_file(mbox_path)?;
    let mbox_entries: Vec<MboxEntry> = split_mbox_entries(&mbox_content)?;

    let mut all_messages: Vec<Message> = Vec::new();
    let total_entries: usize = mbox_entries.len();

    for (index, entry) in mbox_entries.iter().enumerate() {
        all_messages.extend(entry.messages.clone());

        if export_attachments {
            let attachments_folder: PathBuf = output_path.join("attachments");
            for attachment in &entry.attachments {
                write_attachment_to_file(attachment, attachments_folder.to_str().unwrap())?;
            }
        }

        progress_tx.send((index + 1) as f32 / total_entries as f32).unwrap();
    }

    let csv_path: PathBuf = output_path.join("messages.csv");
    write_messages_to_csv(&all_messages, csv_path.to_str().unwrap())?;

    progress_tx.send(1.0).unwrap();

    Ok(())
}

pub fn run_ui() -> Result<(), eframe::Error> {
    let options: NativeOptions = NativeOptions {
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
            Ok(Box::new(MboxExtractorApp::default()))
        }),
    )
}