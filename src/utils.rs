use bevy::math::IVec3;
use rand::{thread_rng, Rng};

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

pub fn random_cells(bounds: i32, probability: f64) -> impl Iterator<Item = (usize, usize, usize)> {
    (0..bounds as usize).flat_map(move |x| {
        (0..bounds as usize).flat_map(move |y| {
            (0..bounds as usize).filter_map(move |z| {
                if thread_rng().gen::<f64>() < probability {
                    Some((x, y, z))
                } else {
                    None
                }
            })
        })
    })
}
