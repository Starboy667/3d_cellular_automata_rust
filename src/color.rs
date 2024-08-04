use bevy::{
    color::{Color, ColorToComponents},
    math::IVec3,
};

use crate::utils::distance;

#[derive(PartialEq, Clone)]
pub enum ColorHandler {
    // rgb pos
    Rgb,
    // dist center
    ColorPalette,
    StateShading,
    NeighborhoodDensity,
}

impl ColorHandler {
    pub fn get_color(
        &self,
        pos: &IVec3,
        bounds: &i32,
        palette: &Vec<[f32; 4]>,
        state: u8,
        state_max: u8,
        neighbors: u8,
    ) -> [f32; 4] {
        match self {
            ColorHandler::Rgb => Color::linear_rgba(
                pos.x as f32 / *bounds as f32,
                pos.y as f32 / *bounds as f32,
                pos.z as f32 / *bounds as f32,
                1.0,
            )
            .to_linear()
            .to_f32_array(),
            ColorHandler::ColorPalette => {
                let center: IVec3 = IVec3::new(*bounds / 2, *bounds / 2, *bounds / 2);
                let distance = distance(pos, &center);
                let max_distance = ((bounds / 2) as f32)
                    .hypot((bounds / 2) as f32)
                    .hypot((bounds / 2) as f32);
                let normalized_distance = distance / max_distance;
                [
                    palette[0][0] + (palette[1][0] - palette[0][0]) * normalized_distance,
                    palette[0][1] + (palette[1][1] - palette[0][1]) * normalized_distance,
                    palette[0][2] + (palette[1][2] - palette[0][2]) * normalized_distance,
                    1.0,
                ]
            }
            ColorHandler::StateShading => {
                let normalized_state: f32 = state as f32 / state_max as f32;
                let green = 1.0 - normalized_state;
                [1.0, green, 0.0, 1.0]
            }
            ColorHandler::NeighborhoodDensity => {
                let max_neighbors = 9 + 8 + 9; // Moore
                let normalized = neighbors as f32 / max_neighbors as f32;
                let red = normalized;
                let blue = 1.0 - normalized;
                [red, 0.0, blue, 1.0]
            }
        }
    }
}
