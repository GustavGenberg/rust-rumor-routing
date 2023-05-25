use crate::position::Position;

#[derive(Clone)]
pub struct Event {
    pub id: u32,
    pub tick: u32,
    pub position: Position,
}

impl Event {
    pub fn new(id: u32, tick: u32, position: Position) -> Self {
        Self {
            id,
            tick,
            position
        }
    }
}