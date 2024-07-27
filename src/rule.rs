use bevy::math::IVec3;

pub struct Rule {
    method: RuleMethod,
    states: u8,
    // TODO optimize
    alive: Vec<u8>,
    dead: Vec<u8>,
}

pub enum RuleMethod {
    Moore,
    VonNeumann,
}

static MOORE: [IVec3; 3] = [
    IVec3::new(-1, -1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, -1, -1),
];

static VONNEUMANN: [IVec3; 3] = [
    IVec3::new(-1, -1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, -1, -1),
];

impl Rule {
    pub fn new(method: RuleMethod) -> Self {
        Self { method }
    }

    pub fn get_neighbors_iter(&self) -> &'static [IVec3] {
        match self.method {
            RuleMethod::Moore => &MOORE[..],
            RuleMethod::VonNeumann => &VONNEUMANN[..],
        }
    }
}
