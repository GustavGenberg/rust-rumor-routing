use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use crate::position::Position;
use crate::event::Event;
use crate::node::{Node, NodeOptions};
use crate::network::Network;

#[derive(Debug)]
pub struct SensorNetworkOptions {
    pub event_probability: u32,
    pub agent_probability: u32,
    pub agent_max_hops: u32,
    pub request_ticks: u32,
    pub request_max_hops: u32,
    pub request_retry_multiplier: u32,
    pub neighbour_range: f64,
}

pub struct SensorNetwork {
    options: SensorNetworkOptions,
    network: Network,
    nodes: HashMap<u32, Node>,
    request_source_nodes: Vec<u32>,
    node_event_ids: Vec<u32>,
    tick: u32,
}

impl SensorNetwork {
    pub fn update(&mut self) -> u32 {
        self.tick += 1;

        let mut rng = rand::thread_rng();

        for (_, node) in &mut self.nodes {
            if rng.gen_range(0..self.options.event_probability) == 0 {
                let id = rng.gen::<u32>();
                
                self.node_event_ids.push(id);
                
                node.detect_event(
                    &mut self.network,
                    self.tick,
                    Event::new(
                        id,
                        self.tick,
                        node.position.clone()
                    )
                )
            }
        }

        if self.tick > 0 && self.tick % self.options.request_ticks == 0 && self.node_event_ids.len() > 0 {
            for id in &self.request_source_nodes {
                self.nodes.get_mut(id).unwrap().send_request(
                    &mut self.network,
                    self.tick,
                    *self.node_event_ids
                        .choose(&mut rng)
                        .unwrap()
                );
            }
        }
        
        let mut answers_received = 0;

        for node in self.nodes.values_mut() {
            answers_received += node.update(&mut self.network, self.tick);
        }

        answers_received
    }

    pub fn new(positions: Vec<Position>, options: SensorNetworkOptions) -> Self {
        println!("running with options: {:?}", &options);

        let mut sensornetwork = Self {
            options,
            network: Network::new(),
            nodes: HashMap::new(),
            request_source_nodes: Vec::new(),
            node_event_ids: Vec::new(),
            tick: 0
        };

        let mut rng = rand::thread_rng();

        for position in positions {
            let id = rng.gen::<u32>();

            sensornetwork.nodes.insert(
                id,
                Node::new(
                    id,
                    position,
                    NodeOptions {
                        agent_probability: sensornetwork.options.agent_probability,
                        agent_max_hops: sensornetwork.options.agent_max_hops,
                        request_max_hops: sensornetwork.options.request_max_hops,
                        request_retry_multiplier: sensornetwork.options.request_retry_multiplier,
                    }
                )
            );
        }

        let mut node_neighbours = HashMap::new();

        for node in sensornetwork.nodes.values() {
            for _node in sensornetwork.nodes.values() {
                if _node.id != node.id && node.position.get_distance_to(&_node.position) <= sensornetwork.options.neighbour_range {
                    if !node_neighbours.contains_key(&node.id) {
                        node_neighbours.insert(node.id, Vec::new());
                    }

                    node_neighbours.get_mut(&node.id).unwrap().push(_node.id);
                }
            }
        }

        for (id, neighbour_nodes) in node_neighbours {
            let node = sensornetwork.nodes.get_mut(&id).unwrap();
            node.neighbour_nodes = neighbour_nodes;
        }

        for _ in 0..4 {
            sensornetwork.request_source_nodes.push(
                *sensornetwork.nodes.keys()
                    .skip(rand::thread_rng().gen_range(0..sensornetwork.nodes.len()))
                    .next()
                    .unwrap()
            );
        }

        sensornetwork
    }
}