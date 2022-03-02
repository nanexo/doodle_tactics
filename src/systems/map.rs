use std::ops::DerefMut;

use pathfinding::prelude::dijkstra;

use crate::prelude::*;


pub fn map_system(
    mut units_query: Query<(Entity, &mut Position, &UnitType, &Party, Option<&MovingFromCell>, Option<&Attacking>)>,
    mut commands: Commands,
)
{

    let units : Vec<(Entity, &Position, &Party)> = units_query.iter()
        .map(|(e, pos, _, party, _, _)| (e, pos, party))
        .collect();

    let unit_positions: Vec<&Position> = units.iter().map(|(_, pos, _)| *pos).collect();


    for (entity, mut pos, unit_type, party, is_moving, is_attacking) in units_query.iter_mut() {
        
        // unit is not idle, go to next
        if is_moving.is_some() || is_attacking.is_some() {
            continue;
        }

        let closest_enemy_unit = units
            .iter()
            .filter(|(_, _, filter_party)| **filter_party != *party )
            .map(|(e, map_pos, _)| (e, map_pos, pos.distance(map_pos)))
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        if let Some(closest_player_unit) = closest_enemy_unit {
            let unit_range= match unit_type {
                &UnitType::Melee => 1,
                &UnitType::Ranged { range } => range,
            };
            if closest_player_unit.2 > unit_range as f32 {
                let target_unit_position = *closest_player_unit.1;
                println!("target_unit_position {:?}", target_unit_position);
                // pathfind
                let result = dijkstra(pos.deref_mut(), |p| {
                    let mut successors = p.successors();
                    successors.retain(|p| !unit_positions.contains(&p) || p == target_unit_position);
                    successors.into_iter().map(|p| (p, 1)).collect::<Vec<(Position, usize)>>()
                }, |p| p == target_unit_position);


                if let Some((path, _)) = result {
                    let new_pos = path.iter().nth(1).unwrap();
                    println!("new_pos: {:?}", new_pos);

                    commands.entity(entity)
                        .insert(MovingFromCell { position: *pos, time: 0. });
    
                    pos.assign(&new_pos);
                } else {
                    println!("no path found");
                }
            }
            else
            {
                commands.entity(entity)
                    .insert(Attacking::with_target(*closest_player_unit.0));
            }
        }
    }
}
pub fn moving_system(
    mut moving_units_query: Query<(Entity, &Position, &mut Transform, &mut MovingFromCell)>,
    time: Res<Time>,
    mut commands: Commands
)
{
    for (e, position, mut transform, mut moving) in moving_units_query.iter_mut() {
        let current_time = moving.time + time.delta_seconds();
        const ANIMATION_TIME: f32 = 1.5;
        if current_time > ANIMATION_TIME {
            commands.entity(e).remove::<MovingFromCell>();
        } else {
            let start = moving.position.to_translation();
            let end = position.to_translation();

            transform.translation = start.lerp(end, current_time / ANIMATION_TIME);
            moving.time = current_time;
        }
    }
}

pub fn attack_system(
    mut attacking_units_query: Query<(Entity, &Position, &mut Attacking)>,
    mut transform_query: Query<&mut Transform>,
    time: Res<Time>,
    entities: &Entities,
    mut commands: Commands
)
{
    for (e, position, mut attacking) in attacking_units_query.iter_mut() {

        const ATTACK_SPEED: f32 = 0.75;

        // verify target still lives
        if entities.contains(attacking.target) {
            let current_time = attacking.time + time.delta_seconds();

            if current_time > ATTACK_SPEED {
                attacking.time = 0.;
                continue;
            }

            if let Ok(target_transform) = transform_query.get(attacking.target) {
                if let Ok(mut unit_transform) = transform_query.get_mut(e) {
                    const MOVE_RANGE : f32 = 0.3;
                    let start_position = position.to_translation();
                    let end_position = target_transform.translation;
                    let animation_position = {
                        let position = current_time / ATTACK_SPEED;
                        if position > 0.5 {
                            0.5 - (position - 0.5)
                        } else {
                            position
                        }
                    };
    
                    unit_transform.translation = start_position.lerp(end_position, animation_position * MOVE_RANGE)
                } 

            }
            attacking.time = current_time;

        } else {
            // target died, remove attacking component
            commands.entity(e).remove::<Attacking>();
        }
    }
}