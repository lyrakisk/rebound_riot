use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct Scored {
    #[event_target]
    pub scorer: Entity,
}
