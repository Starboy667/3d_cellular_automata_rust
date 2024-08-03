use bevy::{
    app::{Plugin, Update},
    prelude::{Query, ResMut},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::{render::InstanceMaterialData, sim::Sims};

pub fn update_ui(
    mut query: Query<&mut InstanceMaterialData>,
    mut this: ResMut<Sims>,
    mut contexts: EguiContexts,
) {
    let mut bounds = this.bounds;
    egui::Window::new("Simulation").show(contexts.ctx_mut(), |ui| {
        let old_bounds = bounds;
        ui.add(egui::Slider::new(&mut bounds, 0..=100).text("bounds"));
        if bounds != old_bounds {
            this.set_size(bounds);
        }
    });
    // let instance_data = &mut query.iter_mut().next().unwrap().0;
    // let rule = this.rule_handler.take().unwrap();
    // this.logic_handler.update(&rule);
    // let mut renderer = this.render_handler.take().unwrap();
    // this.logic_handler.render(&mut renderer);
    // instance_data.truncate(0);
    // for i in 0..renderer.cell_count() as usize {
    //     let value = renderer.values[i];
    //     if value == 0 {
    //         continue;
    //     }
    //     let pos = index_to_pos(i, this.bounds);
    //     instance_data.push(InstanceData {
    //         position: (pos - center(this.bounds)).as_vec3(),
    //         scale: 1.0,
    //         color: (Color::linear_rgba(
    //             pos.x as f32 / this.bounds as f32,
    //             pos.y as f32 / this.bounds as f32,
    //             pos.z as f32 / this.bounds as f32,
    //             1.0,
    //         )
    //         .to_linear()
    //         .to_f32_array()),
    //     });
    // }
    // this.render_handler = Some(renderer);
    // this.rule_handler = Some(rule);
}

pub struct SimUIPlugin;
impl Plugin for SimUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin).add_systems(Update, update_ui);
    }
}
