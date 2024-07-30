use egui::{Context, InputState};
use std::time::Duration;

use super::app::MboxExtractorApp;

/// Handles the smooth transitions for the progress bar, text fade, and button hover effects.
pub fn update_animations(app: &mut MboxExtractorApp, ctx: &Context) {
    let dt: f32 = ctx.input(|i: &InputState| i.stable_dt).min(0.1);

    // Update animated progress
    if let Some(progress) = app.progress {
        app.animated_progress += (progress - app.animated_progress) * dt * 5.0;
        app.animated_progress = app.animated_progress.clamp(0.0, 1.0);
    }

    if !app.processing && !app.result.is_empty() {
        app.result_animation += dt * 2.0;
        app.result_animation = app.result_animation.min(1.0);
    }

    if let Some(start_time) = app.result_start_time {
        if start_time.elapsed() > Duration::from_secs(5) {
            app.result_animation -= dt * 2.0;
            app.result_animation = app.result_animation.max(0.0);

            if app.result_animation == 0.0 {
                app.result = String::new();
                app.result_start_time = None;
            }
        }
    }

    // Update button animations
    for i in 0..3 {
        let target: f32 = if app.button_hover_states[i] { 1.0 } else { 0.0 };
        app.button_animations[i] += (target - app.button_animations[i]) * dt * 10.0;
        app.button_animations[i] = app.button_animations[i].clamp(0.0, 1.0);
    }
}