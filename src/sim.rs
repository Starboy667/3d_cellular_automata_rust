use bevy::{
    app::{Plugin, Update},
    color::{Color, ColorToComponents},
    prelude::{Query, Res, ResMut, Resource},
    time::{Time, Timer},
};

use crate::{
    color::ColorHandler,
    logic,
    render::{CellRenderer, InstanceData, InstanceMaterialData},
    rule,
    utils::{center, index_to_pos},
};

#[derive(Resource)]
pub struct Sims {
    logic_handler: logic::Logic,
    render_handler: Option<Box<CellRenderer>>,
    pub rule_handler: Option<Box<rule::Rule>>,
    pub color_handler: ColorHandler,
    pub bounds: i32,
    setup: bool,
    pub glow: bool,
    pub color_palette: Vec<Color>,
    pub update_timer: Timer,
    pub rule_preset: Vec<rule::RulePreset>,
}

impl Sims {
    pub fn new() -> Self {
        // let tmp_rule = rule::Rule::new(rule::RuleMethod::Moore, vec![4], vec![4], 5);
        let tmp_rule =
            // rule::Rule::new(rule::RuleMethod::Moore, vec![2, 6, 9], vec![4, 6, 8, 9], 10);
        // rule::Rule::new(rule::RuleMethod::Moore, vec![4], vec![4], 5);
        rule::Rule::new(rule::RuleMethod::Moore, vec![5], vec![4, 6, 9, 10, 11, 16, 17, 18, 19, 20, 21, 22, 23, 24], 35);
        let rule = Some(Box::new(tmp_rule));
        Self {
            logic_handler: logic::Logic::new(),
            render_handler: Some(Box::new(CellRenderer::new(64))),
            rule_handler: rule,
            color_handler: ColorHandler::Rgb,
            color_palette: vec![Color::srgb(0.0, 1.0, 1.0), Color::srgb(1.0, 0.0, 0.0)],
            bounds: 64,
            setup: false,
            glow: true,
            update_timer: Timer::from_seconds(0.0, bevy::time::TimerMode::Repeating),
            rule_preset: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.set_size(self.bounds);
    }

    pub fn set_size(&mut self, bounds: i32) {
        let rule = self.rule_handler.as_ref().unwrap();
        self.bounds = bounds;
        self.logic_handler.set_size(bounds);
        self.logic_handler.make_some_noise(&rule);
        self.render_handler.as_mut().unwrap().set_size(bounds);
    }

    fn setup_sim(&mut self) {
        let rule = self.rule_handler.as_ref().unwrap();
        self.logic_handler.set_size(self.bounds);
        self.logic_handler.make_some_noise(&rule);
        self.render_handler.as_mut().unwrap().set_size(self.bounds);
    }

    fn get_presets(&mut self) {
        self.rule_preset = rule::RulePreset::get_presets();
    }

    pub fn load_rule_preset(&mut self, index: usize) {
        self.rule_handler = Some(Box::new(self.rule_preset[index].rule.clone()));
        self.color_palette = self.rule_preset[index].color_palette.clone();
        self.color_handler = self.rule_preset[index].color_handler.clone();
        self.set_size(self.bounds);
    }
}

pub fn update(
    mut query: Query<&mut InstanceMaterialData>,
    mut this: ResMut<Sims>,
    time: Res<Time>,
) {
    this.update_timer.tick(time.delta());
    if !this.setup {
        this.setup_sim();
        this.get_presets();
        this.setup = true;
    }
    if !this.update_timer.finished() {
        return;
    }
    let instance_data = &mut query.iter_mut().next().unwrap().0;
    let rule = this.rule_handler.take().unwrap();
    this.logic_handler.update(&rule);
    let mut renderer = this.render_handler.take().unwrap();
    this.logic_handler.render(&mut renderer);
    instance_data.truncate(0);
    for i in 0..renderer.cell_count() as usize {
        let value = renderer.values[i];
        let neighbors = renderer.neighbors[i];
        if value == 0 {
            continue;
        }
        let pos = index_to_pos(&i, &this.bounds);
        instance_data.push(InstanceData {
            position: (pos - center(this.bounds)).as_vec3(),
            scale: 1.0,
            color: this.color_handler.get_color(
                &pos,
                &this.bounds,
                &this
                    .color_palette
                    .iter()
                    .map(|color| color.to_linear().to_f32_array())
                    .collect(),
                value,
                rule.states,
                neighbors,
            ),
            emissive: if this.glow {
                1.0 + 10.0 * value as f32 / rule.states as f32
            } else {
                1.0
            },
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
