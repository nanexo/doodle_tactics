use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct MovingFromCell {
    pub position: Position,
    pub time: f32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum UnitType {
    Melee,
    Ranged { range: i32 },
}

#[derive(Component, PartialEq)]
pub enum Party {
    Player,
    Enemy,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn assign(&mut self, other: &Position) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn distance(&self, other: &Position) -> f32 {
        Vec2::new(self.x as f32, self.y as f32).distance(Vec2::new(other.x as f32, other.y as f32))
    }

    pub fn to_translation(&self) -> Vec3 {
        const TOP: f32 = 320.;
        const LEFT: f32 = -170.;
        const STEP: f32 = 127.;
        Vec3::new(LEFT + (self.x as f32) * STEP, TOP - (self.y as f32) * STEP, 0.)
    }

    pub fn successors(&self) -> Vec<Position> {
        let &Position {x, y }= self;
        vec![
            Position::new(x, y + 1),
            Position::new(x, y - 1),
            Position::new(x + 1, y),
            Position::new(x - 1, y),
        ]
    }

}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}



#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Attacking {
    pub target: Entity,
    pub time: f32,
}


impl Attacking {
    pub fn with_target(target: Entity) -> Self {
        Attacking {
            target,
            time: 0.
        }
    }
}
