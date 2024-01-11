use std::slice::EscapeAscii;

use crate::{camera::Camera, prelude::*};

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]

pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    //#[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    //#[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        
        let mut did_something = false;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        if !did_something {
            if let Ok(mut health) = ecs
            .entry_mut(player_entity)
            .unwrap()
            .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max,health.current+1);
            }
        }

        *turn_state = TurnState::PlayerTurn;

        // players.iter(ecs).for_each(|(entity, pos)| {
        //     let destination = *pos + delta;
        //     commands.push((
        //         (),
        //         WantsToMove {
        //             entity: *entity,
        //             destination,
        //         },
        //     ));
        // });

        // if delta.x != 0 || delta.y != 0 {
        //     let mut players = <&mut Point>::query().filter(component::<Player>());
        //     players.iter_mut(ecs).for_)each(|pos| {
        //         let destination = *pos + delta;
        //         if map.can_enter_tile(destination) {
        //             *pos = destination;
        //             camera.on_player_move(destination);
        //             *turn_state = TurnState::PlayerTurn;
        //         }
        //     });
        // }
    }
}
