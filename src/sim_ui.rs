use bevy::{
    app::{Plugin, Update},
    color::Color,
    prelude::ResMut,
};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts, EguiPlugin,
};

use crate::{color::ColorHandler, sim::Sims};

fn color_picker(ui: &mut egui::Ui, color: &mut Color) {
    let old_color = color.to_linear();
    let mut new_palette: Color32 = Color32::from_rgb(
        (old_color.red * 255.0) as u8,
        (old_color.green * 255.0) as u8,
        (old_color.blue * 255.0) as u8,
    );
    egui::color_picker::color_edit_button_srgba(
        ui,
        &mut new_palette,
        egui::color_picker::Alpha::Opaque,
    );
    *color = Color::linear_rgba(
        new_palette.r() as f32 / 255.0,
        new_palette.g() as f32 / 255.0,
        new_palette.b() as f32 / 255.0,
        1.0,
    );
}

pub fn update_ui(mut this: ResMut<Sims>, mut contexts: EguiContexts) {
    // TODO rules
    // TODO states
    // TODO reset
    // TODO simulator
    let mut bounds = this.bounds;
    egui::Window::new("Simulation").show(contexts.ctx_mut(), |ui| {
        // BOUNDS
        let old_bounds = bounds;
        ui.add(egui::Slider::new(&mut bounds, 30..=300).text("bounds"));
        if bounds != old_bounds {
            this.set_size(bounds);
        }
        // SPEED
        let mut speed = this.update_timer.duration().as_secs_f32();
        ui.add(egui::Slider::new(&mut speed, 0.0..=0.5).text("speed"));
        if speed != this.update_timer.duration().as_secs_f32() {
            this.update_timer
                .set_duration(std::time::Duration::from_secs_f32(speed));
        }

        // COLOR
        ui.label("Color mode");
        ui.checkbox(&mut this.glow, "Glow");
        ui.horizontal(|ui| {
            ui.radio_value(&mut this.color_handler, ColorHandler::Rgb, "RGB");
            ui.radio_value(
                &mut this.color_handler,
                ColorHandler::ColorPalette,
                "ColorPalette",
            );
            ui.radio_value(
                &mut this.color_handler,
                ColorHandler::StateShading,
                "StateShading",
            );
            ui.radio_value(
                &mut this.color_handler,
                ColorHandler::NeighborhoodDensity,
                "NeighborhoodDensity",
            );
        });
        if this.color_handler == ColorHandler::ColorPalette {
            ui.label("Color palette");
            color_picker(ui, &mut this.color_palette[0]);
            color_picker(ui, &mut this.color_palette[1]);
        }
    });
}

pub struct SimUIPlugin;
impl Plugin for SimUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin).add_systems(Update, update_ui);
    }
}
