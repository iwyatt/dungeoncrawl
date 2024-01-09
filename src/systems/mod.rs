mod entity_render;
mod map_render;
mod player_input;
mod collisions;
mod random_move;
use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush() // Waits for executing systems to complete, and the flushes all outstanding system command buffers.
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}
