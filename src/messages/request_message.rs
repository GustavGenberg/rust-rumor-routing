#[derive(Clone)]
pub struct RequestMessage {
    pub tick: u32,
    pub hops: u32,
    pub route: Vec<u32>,
    pub id: u32,
    pub sequence: u32,
    pub event_id: u32,
    pub is_route_found: bool,
}

impl RequestMessage {
    pub fn new(tick: u32, hops: u32, route: Vec<u32>, id: u32, sequence: u32, event_id: u32, is_route_found: bool) -> Self {
        Self {
            tick,
            hops,
            route,
            id,
            sequence,
            event_id,
            is_route_found
        }
    }
}