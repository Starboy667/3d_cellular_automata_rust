use bevy::math::IVec3;

pub fn index_to_pos(index: &usize, bounds: &i32) -> IVec3 {
    IVec3::new(
        *index as i32 % *bounds,
        *index as i32 / *bounds % *bounds,
        *index as i32 / *bounds / *bounds,
    )
}

pub fn out_of_bounds(pos: &IVec3, bounds: &i32) -> bool {
    pos.x < 0 || pos.y < 0 || pos.z < 0 || pos.x >= *bounds || pos.y >= *bounds || pos.z >= *bounds
}
