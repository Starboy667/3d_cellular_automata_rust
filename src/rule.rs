use bevy::{color::Color, math::IVec3};

use crate::color::ColorHandler;

#[derive(Debug, Clone)]
pub struct Rule {
    method: RuleMethod,
    pub states: u8,
    pub birth: [bool; 27],
    pub survive: [bool; 27],
}

#[derive(Debug, Clone)]
pub enum RuleMethod {
    Moore,
    VonNeumann,
}

static VONNEUMANN: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];

static MOORE: [IVec3; 26] = [
    IVec3::new(-1, -1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, -1, -1),
    IVec3::new(-1, 0, -1),
    IVec3::new(0, 0, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(-1, 1, -1),
    IVec3::new(0, 1, -1),
    IVec3::new(1, 1, -1),
    IVec3::new(-1, -1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(1, -1, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 1, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(1, 1, 0),
    IVec3::new(-1, -1, 1),
    IVec3::new(0, -1, 1),
    IVec3::new(1, -1, 1),
    IVec3::new(-1, 0, 1),
    IVec3::new(0, 0, 1),
    IVec3::new(1, 0, 1),
    IVec3::new(-1, 1, 1),
    IVec3::new(0, 1, 1),
    IVec3::new(1, 1, 1),
];

impl Rule {
    pub fn new(method: RuleMethod, survive: Vec<u8>, birth: Vec<u8>, states: u8) -> Self {
        let mut survive_arr = [false; 27];
        let mut birth_arr = [false; 27];

        for i in survive {
            survive_arr[i as usize] = true;
        }
        for i in birth {
            birth_arr[i as usize] = true;
        }
        Self {
            method,
            birth: birth_arr,
            survive: survive_arr,
            states,
        }
    }

    pub fn get_neighbors_iter(&self) -> &'static [IVec3] {
        match self.method {
            RuleMethod::Moore => &MOORE[..],
            RuleMethod::VonNeumann => &VONNEUMANN[..],
        }
    }
}

#[derive(Clone)]
pub struct RulePreset {
    pub rule: Rule,
    pub name: String,
    pub color_palette: Vec<Color>,
    pub color_handler: ColorHandler,
}

impl RulePreset {
    pub fn get_presets() -> Vec<Self> {
        vec![
            Self {
                name: "builder".into(),
                rule: Rule::new(RuleMethod::Moore, vec![2, 6, 9], vec![4, 6, 8, 9, 10], 10),
                color_handler: ColorHandler::ColorPalette { /* Initialize based on your logic */ },
                color_palette: vec![Color::srgb(1.0, 1.0, 0.0), Color::srgb(1.0, 0.0, 0.0)], // Assuming YELLOW and RED
            },
            Self {
                name: "VN pyramid".into(),
                rule: Rule::new(
                    RuleMethod::VonNeumann,
                    vec![0, 1, 2, 3, 7, 8, 9, 11, 13, 18, 21, 22, 24, 26],
                    vec![4, 13, 17, 20, 21, 22, 23, 24, 26],
                    2,
                ),
                color_handler: ColorHandler::ColorPalette { /* Initialize based on your logic */ },
                color_palette: vec![Color::srgb(0.0, 1.0, 0.0), Color::srgb(0.0, 0.0, 1.0)], // Assuming GREEN and BLUE
            },
            Self {
                name: "fancy snancy".into(),
                rule: Rule::new(
                    RuleMethod::Moore,
                    vec![0, 1, 2, 3, 7, 8, 9, 11, 13, 18, 21, 22, 24, 26],
                    vec![4, 13, 17, 20, 21, 22, 23, 24, 26],
                    4,
                ),
                color_handler: ColorHandler::StateShading { /* Initialize based on your logic */ },
                color_palette: vec![Color::srgb(1.0, 0.0, 0.0), Color::srgb(0.0, 0.0, 1.0)], // Assuming GREEN and BLUE
            },
            Self {
                name: "Clouds 1".into(),
                rule: Rule::new(
                    RuleMethod::Moore,
                    vec![13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26],
                    vec![13, 14, 17, 18, 19],
                    2,
                ),
                color_handler: ColorHandler::NeighborhoodDensity,
                color_palette: vec![Color::srgb(1.0, 0.0, 0.0), Color::srgb(0.0, 0.0, 1.0)],
            },
            Self {
                name: "Crystal Growth 1".into(),
                rule: Rule::new(RuleMethod::Moore, vec![0, 1, 2, 3, 4, 5, 6], vec![1, 3], 2),
                color_handler: ColorHandler::NeighborhoodDensity,
                color_palette: vec![Color::srgb(1.0, 0.0, 0.0), Color::srgb(0.0, 0.0, 1.0)],
            },
        ]
    }
}
