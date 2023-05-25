use std::collections::HashMap;
use rand::Rng;
use rand::prelude::SliceRandom;
use crate::position::Position;
use crate::route::Route;
use crate::network::Network;
use crate::event::Event;
use crate::messages::message::Message;
use crate::messages::agent_message::AgentMessage;
use crate::messages::request_message::RequestMessage;
use crate::messages::response_message::ResponseMessage;

pub struct NodeOptions {
    pub agent_probability: u32,
    pub agent_max_hops: u32,
    pub request_max_hops: u32,
    pub request_retry_multiplier: u32,
}

pub struct Node {
    pub id: u32,
    pub position: Position,
    options: NodeOptions,
    pub neighbour_nodes: Vec<u32>,
    routes: HashMap<u32, Route>,
    events: HashMap<u32, Event>,
    request_messages: HashMap<u32, RequestMessage>
}

impl Node {
    pub fn detect_event(&mut self, network: &mut Network, tick: u32, event: Event) {
        self.routes.insert(
            event.id,
            Route::new(
                event.id,
                self.id,
                0,
            )
        );

        self.events.insert(event.id, event);

        if rand::thread_rng().gen_range(0..self.options.agent_probability) == 0 {
            network.send(
                self.id,
                Message::AgentMessage(
                    AgentMessage::new(
                        tick,
                        0,
                        Vec::new(),
                        HashMap::new(),
                    )
                )
            );
        }
    }

    pub fn send_request(&mut self, network: &mut Network, tick: u32, event_id: u32) {
        let message = RequestMessage::new(
            tick,
            0,
            Vec::new(),
            rand::thread_rng().gen::<u32>(),
            1,
            event_id,
            false
        );

        self.request_messages.insert(
            message.id,
            message.clone()
        );

        network.send(
            self.id,
            Message::RequestMessage(message)
        );
    }

    fn get_neighbour_node(&self, route: &Vec<u32>) -> u32 {
        let possible_nodes: Vec<&u32> = self.neighbour_nodes
            .iter()
            .filter(|id| !route.contains(id))
            .collect();

        let mut rng = rand::thread_rng();

        if possible_nodes.len() > 0 {
            **possible_nodes.choose(&mut rng).unwrap()
        } else {
            *self.neighbour_nodes.choose(&mut rng).unwrap()
        }
    }

    pub fn update(&mut self, network: &mut Network, tick: u32) -> u32 {
        {
            let mut ids_to_remove = Vec::new();
            let mut messages_resent = HashMap::new();

            for (id, message) in &self.request_messages {
                if message.tick + self.options.request_max_hops * self.options.request_retry_multiplier < tick {
                    if message.sequence > 1 {
                        ids_to_remove.push(*id);
                    } else {
                        let _message = RequestMessage::new(
                            tick,
                            message.hops,
                            message.route.clone(),
                            message.id,
                            message.sequence + 1,
                            message.event_id,
                            false
                        );
                
                        messages_resent.insert(
                            _message.id,
                            _message.clone()
                        );
                
                        network.send(
                            self.id,
                            Message::RequestMessage(_message)
                        );
                    }
                }
            }

            for id in ids_to_remove {
                self.request_messages.remove(&id);
            }

            for (id, message) in messages_resent {
                *self.request_messages.get_mut(&id).unwrap() = message;
            }
        }

        let message = network.receive(tick, self.id);

        let mut answers_received = 0;

        if let Some(message) = message {
            match message {
                Message::AgentMessage(message) => {
                    let mut routes: HashMap<u32, Route> = HashMap::new();

                    for route in message.routes.values() {
                        let local_route = self.routes.get_mut(&route.event_id);

                        if let Some(local_route) = local_route {
                            if route.shortest_distance < local_route.shortest_distance {
                                local_route.node_id = route.node_id;
                                local_route.shortest_distance = route.shortest_distance;
                            }
                        } else {
                            self.routes.insert(
                                route.event_id,
                                route.clone()
                            );
                        }
                    }

                    for route in self.routes.values() {
                        routes.insert(
                            route.event_id,
                            Route::new(
                                route.event_id,
                                self.id,
                                route.shortest_distance + 1
                            )
                        );
                    }

                    if message.hops < self.options.agent_max_hops {
                        let receiving_node_id = self.get_neighbour_node(&message.route);
                        
                        network.send(
                            receiving_node_id,
                            Message::AgentMessage(
                                AgentMessage::new(
                                    tick + 1,
                                    message.hops + 1,
                                    {
                                        let mut route = message.route.clone();
                                        route.push(self.id);
                                        route
                                    },
                                    routes
                                )
                            )
                        );
                    }
                },
                Message::RequestMessage(message) => {
                    let local_event = self.events.get(&message.event_id);

                    if let Some(local_event) = local_event {
                        network.send(
                            self.id,
                            Message::ResponseMessage(
                                ResponseMessage::new(
                                    tick + 1,
                                    0,
                                    Vec::new(),
                                    message.id,
                                    {
                                        let mut route = message.route.clone();
                                        route.reverse();
                                        route
                                    },
                                    message.event_id,
                                    local_event.clone()
                                )
                            )
                        )
                    } else if message.is_route_found || message.hops < self.options.request_max_hops {
                        let local_route = self.routes.get(&message.event_id);

                        let receiving_node_id = if let Some(local_route) = local_route { local_route.node_id } else { self.get_neighbour_node(&message.route) };

                        network.send(
                            receiving_node_id, 
                            Message::RequestMessage(
                                RequestMessage::new(
                                    tick + 1,
                                    message.hops + 1,
                                    {
                                        let mut route = message.route.clone();
                                        route.push(self.id);
                                        route
                                    },
                                    message.id,
                                    message.sequence,
                                    message.event_id,
                                    local_route.is_some()
                                )
                            )
                        )
                    }
                },
                Message::ResponseMessage(message) => {
                    if self.request_messages.contains_key(&message.id) {
                        self.request_messages.remove(&message.id);
                        
                        answers_received += 1;
                        
                        println!(
                            "response received: event_id: {}, tick: {}, x: {}, y: {}, hops: {}", 
                            message.event.id, 
                            message.event.tick, 
                            message.event.position.x, 
                            message.event.position.y,
                            message.hops,
                        );
                    } else {
                        let node_id = message.path.get(0);

                        if let Some(node_id) = node_id {
                            network.send(
                                *node_id,
                                Message::ResponseMessage(
                                    ResponseMessage::new(
                                        tick + 1,
                                        message.hops + 1,
                                        {
                                            let mut route = message.route.clone();
                                            route.push(self.id);
                                            route
                                        },
                                        message.id,
                                        {
                                            let mut path = message.path.clone();
                                            path.remove(0);
                                            path
                                        },
                                        message.event_id,
                                        message.event
                                    )
                                )
                            )
                        }
                    }
                }
            }
        }

        answers_received
    }

    pub fn new(id: u32, position: Position, options: NodeOptions) -> Self {
        Self {
            id,
            position,
            options,
            neighbour_nodes: Vec::new(),
            routes: HashMap::new(),
            events: HashMap::new(),
            request_messages: HashMap::new()
        }
    }
}