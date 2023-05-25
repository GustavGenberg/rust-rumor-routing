use std::collections::HashMap;
use crate::route::Route;

#[derive(Clone)]
pub struct AgentMessage {
    pub tick: u32,
    pub hops: u32,
    pub route: Vec<u32>,
    pub routes: HashMap<u32, Route>
}

impl AgentMessage {
    pub fn new(tick: u32, hops: u32, route: Vec<u32>, routes: HashMap<u32, Route>) -> Self {
        Self {
            tick,
            hops,
            route,
            routes
        }
    }
}