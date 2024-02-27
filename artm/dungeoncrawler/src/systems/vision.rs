use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn vision(ecs: &mut SubWorld, #[resource] map: &mut Map) {
    let player_entity = *<Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    let mut fovs = <(Entity, &Point, &mut FieldOfView)>::query();
    fovs.iter_mut(ecs)
        .filter(|(_, _, fov)| fov.dirty)
        .for_each(|(entity, pos, fov)| {
            fov.points = field_of_view_set(*pos, fov.radius, map);
            fov.dirty = false;
            if *entity == player_entity {
                for pos in &fov.points {
                    let idx = map.point_idx(*pos);
                    map.seen[idx] = true;
                }
            }
        });
}
