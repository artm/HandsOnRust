use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn vision(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut fovs = <(&Point, &mut FieldOfView)>::query();
    fovs.iter_mut(ecs)
        .filter(|(_, fov)| fov.dirty)
        .for_each(|(pos, fov)| {
            fov.points = field_of_view_set(*pos, fov.radius, map);
            fov.dirty = false;
        });
}
