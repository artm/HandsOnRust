use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn motion(
    entity: &Entity,
    motion: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter(motion.destination) {
        commands.add_component(motion.entity, motion.destination);

        if let Ok(entry) = ecs.entry_ref(motion.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(motion.entity, fov.clone_dirty());
            }
            if entry.get_component::<Player>().is_ok() {
                camera.update(motion.destination);
            }
        }
    }
    commands.remove(*entity);
}
