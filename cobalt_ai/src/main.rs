#![recursion_limit="512"]
#![type_length_limit="8388608"]

mod model;
mod data; 
mod inference;

use burn::backend::{Autodiff, Wgpu};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- [train | generate <prompt>]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "train" => {
            type MyBackend = Wgpu;
            type MyAutodiffBackend = Autodiff<MyBackend>;
            let device = burn::backend::wgpu::WgpuDevice::default();
            
            println!("Starting training pipeline...");
            model::trainer::train::<MyAutodiffBackend>(device);
        }
        "generate" => {
            let prompt = if args.len() > 2 { &args[2] } else { "ROMEO:" };
            type MyBackend = Wgpu;
            let device = burn::backend::wgpu::WgpuDevice::default();
            
            println!("Starting generation pipeline...");
            inference::generate::generate_text::<MyBackend>(device, prompt, 200); 
        }
        _ => {
            println!("Unknown command. Use 'train' or 'generate'.");
        }
    }
}