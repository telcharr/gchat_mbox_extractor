use egui::{Ui, RichText, Frame, Color32, Rounding, Stroke, Vec2, Rect, Sense, ProgressBar};

use super::app::MboxExtractorApp;

const BG_COLOR: Color32 = Color32::from_rgb(33, 37, 41);
const ACCENT_COLOR: Color32 = Color32::from_rgb(72,77,83);
const TEXT_COLOR: Color32 = Color32::from_rgb(211,211,212);
const BUTTON_COLOR: Color32 = Color32::from_rgb(52,58,64);
const HOVER_COLOR: Color32 = Color32::from_rgb(72,77,83);

pub enum Action {
    OpenMboxFileDialog,
    OpenOutputFolderDialog,
    ToggleExportAttachments,
    StartProcessing,
    UpdateProgress(f32),
    FinishProcessing,
    SetButtonHoverState(usize, bool),
}

/// Called to render UI elements
pub fn render_ui(app: &MboxExtractorApp, ctx: &egui::Context) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    let frame: Frame = Frame::default()
        .fill(BG_COLOR)
        .inner_margin(20.0);

    egui::CentralPanel::default().frame(frame).show(ctx, |ui: &mut Ui| {
        ui.vertical_centered(|ui: &mut Ui| {
            ui.add_space(20.0);
            ui.heading(RichText::new("GChat MBOX Extractor").color(TEXT_COLOR).size(24.0));
            ui.add_space(30.0);

            if !app.processing {
                actions.extend(render_file_selection(app, ui));
                ui.add_space(20.0);
                actions.extend(render_output_selection(app, ui));
                ui.add_space(20.0);
                actions.extend(render_process_button(app, ui));
            } else {
                render_processing(app, ui);
            }

            ui.add_space(30.0);
            render_result(app, ui);
        });
    });

    actions
}

/// Renders the MBOX file selection button.
fn render_file_selection(app: &MboxExtractorApp, ui: &mut Ui) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    ui.vertical_centered(|ui: &mut Ui| {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(200.0, 40.0), Sense::click_and_drag());
        if animated_button(ui, "Select MBOX File", rect, app.button_animations[0]) && response.clicked() {
            actions.push(Action::OpenMboxFileDialog);
        }
        actions.push(Action::SetButtonHoverState(0, response.hovered()));
        ui.add_space(10.0);
        if let Some(path) = &app.mbox_path {
            ui.label(RichText::new(format!("File: {}", path.file_name().unwrap_or_default().to_string_lossy())).color(TEXT_COLOR));
        }
    });
    actions
}

/// Renders the output selection button.
fn render_output_selection(app: &MboxExtractorApp, ui: &mut Ui) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    ui.vertical_centered(|ui: &mut Ui| {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(200.0, 40.0), Sense::click_and_drag());
        if animated_button(ui, "Select Output Folder", rect, app.button_animations[1]) && response.clicked() {
            actions.push(Action::OpenOutputFolderDialog);
        }
        actions.push(Action::SetButtonHoverState(1, response.hovered()));
        ui.add_space(10.0);
        if let Some(path) = &app.output_path {
            ui.label(RichText::new(format!("Folder: {}", path.file_name().unwrap_or_default().to_string_lossy())).color(TEXT_COLOR));
        }
        ui.add_space(10.0);
        let mut export_attachments: bool = app.export_attachments;
        if ui.checkbox(&mut export_attachments, RichText::new("Export Attachments").color(TEXT_COLOR)).changed() {
            actions.push(Action::ToggleExportAttachments);
        }
    });
    actions
}

/// Renders the process button
fn render_process_button(app: &MboxExtractorApp, ui: &mut Ui) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    ui.vertical_centered(|ui: &mut Ui| {
        if app.mbox_path.is_some() && app.output_path.is_some() {
            let (rect, response) = ui.allocate_exact_size(Vec2::new(200.0, 40.0), Sense::click_and_drag());
            if animated_button(ui, "Process MBOX", rect, app.button_animations[2]) && response.clicked() {
                actions.push(Action::StartProcessing);
            }
            actions.push(Action::SetButtonHoverState(2, response.hovered()));
        }
    });
    actions
}

/// Renders the processing UI with progress bar.
fn render_processing(app: &MboxExtractorApp, ui: &mut Ui) {
    ui.vertical_centered(|ui: &mut Ui| {
        ui.add_space(20.0);
        let progress_bar: ProgressBar = ProgressBar::new(app.animated_progress)
            .animate(true)
            .show_percentage()
            .desired_width(200.0);
        ui.add(progress_bar);
        ui.label(RichText::new("Processing...").color(TEXT_COLOR));
    });
}

/// Renders the result received from the MBOX processor
fn render_result(app: &MboxExtractorApp, ui: &mut Ui) {
    ui.vertical_centered(|ui: &mut Ui| {
        if !app.result.is_empty() {
            let color: Color32 = Color32::from_rgba_unmultiplied(
                TEXT_COLOR.r(),
                TEXT_COLOR.g(),
                TEXT_COLOR.b(),
                (app.result_animation * 255.0) as u8,
            );
            ui.label(RichText::new(&app.result).color(color));
        }
    });
}

/// Custom animated button component.
fn animated_button(ui: &mut Ui, text: &str, rect: Rect, animation: f32) -> bool {
    let lerp: fn(f32, f32, f32) -> f32 = |a: f32, b: f32, t: f32| a + (b - a) * t;
    let fill_color: Color32 = Color32::from_rgb(
        lerp(BUTTON_COLOR.r() as f32, HOVER_COLOR.r() as f32, animation) as u8,
        lerp(BUTTON_COLOR.g() as f32, HOVER_COLOR.g() as f32, animation) as u8,
        lerp(BUTTON_COLOR.b() as f32, HOVER_COLOR.b() as f32, animation) as u8,
    );

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
        lerp(200.0, 255.0, animation) as u8,
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0),
        text_color,
    );

    ui.rect_contains_pointer(rect)
}