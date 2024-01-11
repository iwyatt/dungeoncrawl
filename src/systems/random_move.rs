use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination
            }
        ));
    });
}

// pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
//     let mut movers = <(&mut Point, &MovingRandomly)>::query();
//     movers.iter_mut(ecs).for_each(|(pos, _)| {
//         let mut rng = RandomNumberGenerator::new();
//         let destination = match rng.range(0, 4) {
//             0 => Point::new(-1, 0),
//             1 => Point::new(1, 0),
//             2 => Point::new(0, -1),
//             _ => Point::new(0, 1),
//         } + *pos;

//         if map.can_enter_tile(destination) {
//             *pos = destination;
//         }
//     });
// }
