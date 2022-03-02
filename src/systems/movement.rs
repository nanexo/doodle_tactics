use crate::prelude::*;


pub fn map_system(
    mut units_query: Query<(Entity, &mut Position, &UnitType, &Party, Option<&MovingFromCell>, Option<&Attacking>)>,
    mut commands: Commands,
)
{

    let units : Vec<(Entity, &Position, &Party)> = units_query.iter()
        .map(|(e, pos, _, party, _, _)| (e, pos, party))
        .collect();


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
                // not in range to attack, move towards enemy
                let new_pos: Position;
                if closest_player_unit.1.x != pos.x {
                    // try to move vertically first
                    if closest_player_unit.1.x > pos.x {
                        new_pos = Position::new(pos.x + 1, pos.y);
                    } else {
                        new_pos = Position::new(pos.x - 1, pos.y);
                    }
                } else {
                    if closest_player_unit.1.y > pos.y {
                        new_pos = Position::new(pos.x, pos.y + 1);
                    } else {
                        new_pos = Position::new(pos.x, pos.y - 1);
                    }
                };

                if !units.iter().any(|(_, pos, _)| **pos == new_pos) {

                    println!("new_pos: {:?}", new_pos);

                    commands.entity(entity)
                        .insert(MovingFromCell { position: *pos, time: 0. });
    
                    pos.assign(&new_pos);
                }


            }
            else
            {
                commands.entity(entity)
                    .insert(Attacking { target: *closest_player_unit.0 });
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