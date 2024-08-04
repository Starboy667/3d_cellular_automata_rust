use bevy::math::IVec3;

#[derive(Debug)]
pub struct Rule {
    method: RuleMethod,
    pub states: u8,
    pub birth: [bool; 27],
    pub survive: [bool; 27],
}

#[derive(Debug)]
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
