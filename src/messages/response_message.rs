use crate::event::Event;

#[derive(Clone)]
pub struct ResponseMessage {
    pub tick: u32,
    pub hops: u32,
    pub route: Vec<u32>,
    pub id: u32,
    pub path: Vec<u32>,
    pub event_id: u32,
    pub event: Event,
}

impl ResponseMessage {
    pub fn new(tick: u32, hops: u32, route: Vec<u32>, id: u32, path: Vec<u32>, event_id: u32, event: Event) -> Self {
        Self {
            tick,
            hops,
            route,
            id,
            path,
            event_id,
            event
        }
    }
}