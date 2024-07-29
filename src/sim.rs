use bevy::{
    app::{Plugin, Update},
    color::{Color, ColorToComponents, LinearRgba},
    math::{ivec3, IVec3},
    prelude::{Query, ResMut, Resource},
};

use crate::{
    logic,
    render::{CellRenderer, InstanceData, InstanceMaterialData},
    rule,
};

#[derive(Resource)]
pub struct Sims {
    logic_handler: logic::Logic,
    render_handler: Option<Box<CellRenderer>>,
    rule_handler: Option<Box<rule::Rule>>,
    bounds: i32,
}

impl Sims {
    pub fn new() -> Self {
        let mut logic = logic::Logic::new(64);
        // let tmp_rule = rule::Rule::new(rule::RuleMethod::Moore, vec![4], vec![4], 5);
        let tmp_rule =
            rule::Rule::new(rule::RuleMethod::Moore, vec![2, 6, 9], vec![4, 6, 8, 9], 10);

        logic.update_neighbors(None, &tmp_rule);
        let rule = Some(Box::new(tmp_rule));
        Self {
            logic_handler: logic,
            render_handler: Some(Box::new(CellRenderer::new(64))),
            rule_handler: rule,
            bounds: 64,
        }
    }
}
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

pub fn update(mut query: Query<&mut InstanceMaterialData>, mut this: ResMut<Sims>) {
    let instance_data = &mut query.iter_mut().next().unwrap().0;
    let rule = this.rule_handler.take().unwrap();
    this.logic_handler.update(&rule);
    let mut renderer = this.render_handler.take().unwrap();
    this.logic_handler.render(&mut renderer);
    instance_data.truncate(0);
    for i in 0..renderer.cell_count() as usize {
        let value = renderer.values[i];
        if value == 0 {
            continue;
        }
        let pos = index_to_pos(i, this.bounds);
        instance_data.push(InstanceData {
            position: (pos - center(this.bounds)).as_vec3(),
            scale: 1.0,
            color: LinearRgba::from(Color::hsla(
                pos.x as f32 * 360.0 / this.bounds as f32,
                pos.y as f32 / this.bounds as f32,
                pos.z as f32 / this.bounds as f32,
                1.0,
            ))
            .to_f32_array(),
        });
    }
    this.render_handler = Some(renderer);
    this.rule_handler = Some(rule);
}

pub struct SimsPlugin;
impl Plugin for SimsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Sims::new()).add_systems(Update, update);
    }
}
