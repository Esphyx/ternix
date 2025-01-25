use strum::EnumCount;

mod axis;
mod direction;
mod polarity;
mod rotation;

pub use axis::Axis;
pub use direction::Direction;
pub use polarity::Polarity;
pub use rotation::Rotation;

#[derive(Debug, Default, Clone, Copy)]
pub struct Orientation {
    pub direction: Direction,
    pub rotation: Rotation,
}

#[cfg(test)]
mod tests {
    use crate::{Axis, Direction, Orientation, Polarity, Rotation};

    #[test]
    pub fn it_works() {
        let orientation = Orientation::from((
            Direction::from((Polarity::Positive, Axis::Y)),
            Rotation::Identity,
        ));
        let direction = Direction::from((Polarity::Negative, Axis::Y));

        println!("{:?}", orientation.rotate(direction));
    }
}

impl Orientation {
    pub fn rotate(&self, along: Direction) -> Self {
        let cross = self.direction.cross(along);

        match cross {
            Some(dir) => {
                if dir.axis == Axis::Y {
                    
                }

                todo!()
            }
            None => Self {
                direction: self.direction,
                rotation: match self.direction.polarity.cross(along.polarity) {
                    Polarity::Positive => self.rotation.next(),
                    Polarity::Negative => self.rotation.previous(),
                },
            },
        }
    }

    pub fn combination(&self) -> usize {
        self.rotation as usize + self.direction.combination() * Rotation::COUNT
    }
}

impl From<(Direction, Rotation)> for Orientation {
    fn from(value: (Direction, Rotation)) -> Self {
        let (direction, rotation) = value;
        Self {
            direction,
            rotation,
        }
    }
}

impl EnumCount for Orientation {
    const COUNT: usize = Direction::COUNT * Rotation::COUNT;
}
