use bevy::math::IVec3;

pub struct Rule {
    method: RuleMethod,
    pub states: u8,
    // TODO optimize
    pub alive: [bool; 27],
    pub dead: [bool; 27],
}

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
    pub fn new(method: RuleMethod, alive: Vec<u8>, dead: Vec<u8>, states: u8) -> Self {
        let mut dead_arr = [false; 27];
        let mut alive_arr = [false; 27];

        for i in dead {
            dead_arr[i as usize] = true;
        }
        for i in alive {
            alive_arr[i as usize] = true;
        }
        Self {
            method,
            alive: alive_arr,
            dead: dead_arr,
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
