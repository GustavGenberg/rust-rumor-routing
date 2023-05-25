mod position;
mod route;
mod node;
mod event;
mod messages;
mod sensornetwork;
mod network;

use std::fs;
use std::time::Instant;
use clap::Parser;
use crate::position::Position;
use crate::sensornetwork::{SensorNetwork, SensorNetworkOptions};

#[derive(Parser, Clone)]
pub struct Args {
    #[clap(long, default_value_t = 10000)]
    pub event_probability: u32,
    #[clap(long, default_value_t = 2)]
    pub agent_probability: u32,
    #[clap(long, default_value_t = 50)]
    pub agent_max_hops: u32,
    #[clap(long, default_value_t = 400)]
    pub request_ticks: u32,
    #[clap(long, default_value_t = 45)]
    pub request_max_hops: u32,
    #[clap(long, default_value_t = 8)]
    pub request_retry_multiplier: u32,
    #[clap(long, default_value_t = 15.0)]
    pub neighbour_range: f64,
    #[clap(long, default_value_t = 10000)]
    pub iterations: u32,
    #[clap()]
    pub layout_file_path: String,
}


fn main() {
    let args = Args::parse();

    let layout_contents = fs::read_to_string(args.layout_file_path)
        .expect("could not read layout.");
    let layout_lines = layout_contents.lines();

    let mut positions: Vec<Position> = Vec::new();

    for line in layout_lines.skip(1) {
        let mut coordinates = line.split(',');

        positions.push(
            Position::new(
                coordinates
                    .next().unwrap()
                    .parse().unwrap(),
                coordinates
                    .next().unwrap()
                    .parse().unwrap()
            )
        )
    }

    let mut network = SensorNetwork::new(
        positions,
        SensorNetworkOptions {
            event_probability: args.event_probability,
            agent_probability: args.agent_probability,
            agent_max_hops: args.agent_max_hops,
            request_ticks: args.request_ticks,
            request_max_hops: args.request_max_hops,
            request_retry_multiplier: args.request_retry_multiplier,
            neighbour_range: args.neighbour_range,
        }
    );

    let mut answers_received = 0;
    
    let iterations = args.iterations;

    println!("running {} iterations.", iterations);
    
    let now: Instant = Instant::now();
    for _ in 0..iterations {
        answers_received += network.update();
    }
    let elapsed = now.elapsed();

    println!("done in {:?}, received {} answers.", elapsed, answers_received);
}
