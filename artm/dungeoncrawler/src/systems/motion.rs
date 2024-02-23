use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn motion(
    entity: &Entity,
    motion: &MotionIntent,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter(motion.destination) {
        commands.add_component(motion.entity, motion.destination);

        if ecs
            .entry_ref(motion.entity)
            .expect("Entity that set motion intent exists")
            .get_component::<Player>()
            .is_ok()
        {
            camera.update(motion.destination);
        }
    }
    commands.remove(*entity);
}
