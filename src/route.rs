#[derive(Clone)]
pub struct Route {
    pub event_id: u32,
    pub node_id: u32,
    pub shortest_distance: u32
}

impl Route {
    pub fn new(event_id: u32, node_id: u32, shortest_distance: u32) -> Self {
        Self {
            event_id,
            node_id,
            shortest_distance
        }
    }
}