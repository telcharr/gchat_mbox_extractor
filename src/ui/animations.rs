use egui::{Context, InputState};
use std::time::Duration;

use super::app::{MboxExtractorApp, Action};

const ANIMATION_THRESHOLD: f32 = 0.001;
const PROGRESS_ANIMATION_SPEED: f32 = 6.0;
const RESULT_ANIMATION_SPEED: f32 = 2.0;
const BUTTON_ANIMATION_SPEED: f32 = 10.0;
const RESULT_DISPLAY_DURATION: Duration = Duration::from_secs(5);

/// Updates all animations in the application and returns a vector of actions to be performed
pub fn update_animations(app: &MboxExtractorApp, ctx: &Context) -> Vec<Action> {
    let dt: f32 = ctx.input(|i: &InputState| i.stable_dt).min(0.1);
    let mut actions: Vec<Action> = Vec::new();

    update_progress_animation(app, dt, &mut actions);
    update_result_animation(app, dt, &mut actions);
    update_button_animations(app, dt, &mut actions);

    actions
}

/// Handles the smooth transition of the progress bar animation
fn update_progress_animation(app: &MboxExtractorApp, dt: f32, actions: &mut Vec<Action>) {
    if let Some(progress) = app.progress {
        let new_progress: f32 = if app.processing_complete && app.animated_progress > 0.9999 {
            1.0
        } else {
            (app.animated_progress + (progress - app.animated_progress) * dt * PROGRESS_ANIMATION_SPEED)
                .clamp(0.0, 1.0)
        };
        if (new_progress - app.animated_progress).abs() > ANIMATION_THRESHOLD {
            actions.push(Action::UpdateProgress(new_progress));
        }
    }
}

/// Manages the fade-in and fade-out animation of the result text
fn update_result_animation(app: &MboxExtractorApp, dt: f32, actions: &mut Vec<Action>) {
    let mut new_result_animation: f32 = app.result_animation;
    if !app.processing && !app.result.is_empty() {
        new_result_animation = (new_result_animation + dt * RESULT_ANIMATION_SPEED).min(1.0);
    }

    if let Some(start_time) = app.result_start_time {
        if start_time.elapsed() > RESULT_DISPLAY_DURATION {
            new_result_animation = (new_result_animation - dt * RESULT_ANIMATION_SPEED).max(0.0);
            if new_result_animation == 0.0 {
                actions.push(Action::FinishProcessing);
            }
        }
    }
}

/// Updates the hover animations for all buttons
fn update_button_animations(app: &MboxExtractorApp, dt: f32, actions: &mut Vec<Action>) {
    for i in 0..3 {
        let target: f32 = if app.button_hover_states[i] { 1.0 } else { 0.0 };
        let new_animation: f32 = (app.button_animations[i] + (target - app.button_animations[i]) * dt * BUTTON_ANIMATION_SPEED)
            .clamp(0.0, 1.0);
        if (new_animation - app.button_animations[i]).abs() > ANIMATION_THRESHOLD {
            actions.push(Action::UpdateButtonAnimation(i, new_animation));
        }
    }
}