use bevy::math::{ivec3, IVec3};

pub fn index_to_pos(index: &usize, bounds: &i32) -> IVec3 {
    IVec3::new(
        *index as i32 % *bounds,
        *index as i32 / *bounds % *bounds,
        *index as i32 / *bounds / *bounds,
    )
}

pub fn pos_to_index(pos: &IVec3, bounds: &i32) -> usize {
    (pos.x + pos.y * *bounds + pos.z * *bounds * *bounds) as usize
}

pub fn out_of_bounds(pos: &IVec3, bounds: &i32) -> bool {
    pos.x < 0 || pos.y < 0 || pos.z < 0 || pos.x >= *bounds || pos.y >= *bounds || pos.z >= *bounds
}

pub fn distance(pos1: &IVec3, pos2: &IVec3) -> f32 {
    (((pos1.x - pos2.x).pow(2) + (pos1.y - pos2.y).pow(2) + (pos1.z - pos2.z).pow(2)) as f32).sqrt()
}

pub fn center(bounds: i32) -> IVec3 {
    let center = bounds / 2;
    ivec3(center, center, center)
}
