use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::{models::WalkAction, ActorQueue},
    board::Position,
    pieces::components::Actor,
    player::Player,
    vectors::Vector2Int,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, player_position);
    }
}

#[derive(Debug, Event)]
pub struct PlayerInputReadyEvent;

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::KeyW, Vector2Int::UP),
    (KeyCode::KeyS, Vector2Int::DOWN),
    (KeyCode::KeyA, Vector2Int::LEFT),
    (KeyCode::KeyD, Vector2Int::RIGHT),
];

fn player_position(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        let action = WalkAction(entity, position.v + dir);
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}
