use bevy::{
    app::{Plugin, Update},
    color::{Color, ColorToComponents, LinearRgba},
    math::{ivec3, IVec3},
    prelude::Query,
};

use crate::render::{InstanceData, InstanceMaterialData};

fn index_to_pos(index: usize, bounds: i32) -> IVec3 {
    ivec3(
        index as i32 % bounds,
        index as i32 / bounds % bounds,
        index as i32 / bounds / bounds,
    )
}

pub fn center(bounds: i32) -> IVec3 {
    let center = bounds / 2;
    ivec3(center, center, center)
}

pub fn update(mut query: Query<&mut InstanceMaterialData>) {
    let instance_data = &mut query.iter_mut().next().unwrap().0;
    if (instance_data.len() == 262144) {
        return;
    }
    instance_data.truncate(0);
    println!("{}", instance_data.len());
    let bounds: i32 = 64;
    for i in 0..(bounds * bounds * bounds) as usize {
        let pos = index_to_pos(i, bounds);
        instance_data.push(InstanceData {
            position: (pos - center(bounds)).as_vec3(),
            scale: 1.0,
            color: LinearRgba::from(Color::hsla(
                pos.x as f32 * 360.0 / bounds as f32,
                pos.y as f32 / bounds as f32,
                pos.z as f32 / bounds as f32,
                1.0,
            ))
            .to_f32_array(),
        });
    }

    // for index in 0..renderer.cell_count() {
    //     let value = renderer.values[index];
    //     let neighbors = renderer.neighbors[index];

    //     if value != 0 {
    //         let pos = index_to_pos(index, bounds);
    //         instance_data.push(InstanceData {
    //             position: (pos - utils::center(bounds)).as_vec3(),
    //             scale: 1.0,
    //             color: this
    //                 .color_method
    //                 .color(
    //                     this.color1,
    //                     this.color2,
    //                     rule.states,
    //                     value,
    //                     neighbors,
    //                     utils::dist_to_center(pos, bounds),
    //                 )
    //                 .into(),
    //         });
    //     }
    // }
}

pub struct SimsPlugin;
impl Plugin for SimsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .insert_resource(Sims::new())
            .add_systems(Update, update);
    }
}
