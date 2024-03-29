use bevy::prelude::*;

#[derive(Event)]
pub struct AnimationFinishedEvent {
    pub name: String,
    pub entity: Entity,
}
