#![recursion_limit="512"]
#![type_length_limit="8388608"]

mod model;
mod data; 
mod inference;

use burn::backend::{Autodiff, Wgpu};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("╔══════════════════════════════════════════════╗");
        println!("║         🟢 COBALT AI - From Scratch         ║");
        println!("╚══════════════════════════════════════════════╝");
        println!();
        println!("Usage:");
        println!("  cargo run -- train                    - Train the model");
        println!("  cargo run -- generate <prompt>        - Generate text");
        println!("  cargo run -- chat                     - Interactive chat mode");
        println!();
        println!("Examples:");
        println!("  cargo run -- train");
        println!("  cargo run -- generate \"Hello\"");
        println!("  cargo run -- chat");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "train" => {
            type MyBackend = Wgpu;
            type MyAutodiffBackend = Autodiff<MyBackend>;
            let device = burn::backend::wgpu::WgpuDevice::default();
            
            println!("🚀 Starting training pipeline...");
            println!("📊 Model: Cobalt Transformer (d_model=192, layers=3, heads=4)");
            println!("💾 Data: data/input.txt");
            model::trainer::train::<MyAutodiffBackend>(device);
        }
        "generate" => {
            let prompt = if args.len() > 2 { &args[2] } else { "The meaning of life is" };
            type MyBackend = Wgpu;
            let device = burn::backend::wgpu::WgpuDevice::default();
            
            println!("🎨 Generating text with temperature=0.8, top_k=40...");
            inference::generate::generate_text::<MyBackend>(device, prompt, 200, 0.8, 40);
        }
        "chat" => {
            interactive_chat();
        }
        _ => {
            println!("❌ Unknown command: {}", command);
            println!("Use 'train', 'generate', or 'chat'");
        }
    }
}

fn interactive_chat() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║      💬 Cobalt AI Interactive Chat Mode     ║");
    println!("╚══════════════════════════════════════════════╝");
    println!("Type your messages and press Enter.");
    println!("Type 'exit' or 'quit' to stop.");
    println!("Type 'reset' to clear conversation.");
    println!();
    
    use std::io::{self, Write};
    use burn::backend::Wgpu;
    use crate::data::loader::TextDataset;
    
    let device = WgpuDevice::default();
    let dataset = TextDataset::new("data/input.txt");
    
    let d_model = 192;
    let seq_len = 64;
    let n_heads = 4;
    let n_layers = 3;
    
    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);
    let recorder = CompactRecorder::new();
    
    let record = match recorder.load("models/cobalt_model".into(), &device) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("❌ No trained model found! Run 'cargo run -- train' first.");
            return;
        }
    };
    
    let model: CobaltModel<Wgpu> = config.init(&device).load_record(record);
    
    let mut conversation = String::new();
    
    loop {
        print!("\n🟢 You: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "exit" | "quit" => {
                println!("👋 Goodbye!");
                break;
            }
            "reset" => {
                conversation.clear();
                println!("🧹 Conversation reset!");
                continue;
            }
            _ => {
                conversation.push_str(&format!("Human: {}\nAI:", input));
                
                let prompt = &conversation;
                let tokens = dataset.tokenizer.encode(prompt);
                let seq_len = 64;
                
                let mut response = String::new();
                let mut current_tokens = tokens;
                
                for _ in 0..100 {
                    let (context_start, actual_seq_len) = if current_tokens.len() > seq_len {
                        (current_tokens.len() - seq_len, seq_len)
                    } else {
                        (0, current_tokens.len())
                    };
                    
                    let context = &current_tokens[context_start..];
                    let input_tensor = Tensor::<Wgpu, 2, Int>::from_data(
                        TensorData::new(context.to_vec(), [1, actual_seq_len]),
                        &device,
                    );
                    
                    let output = model.forward(input_tensor);
                    let [_, seq_dim, vocab_dim] = output.dims();
                    let logits = output.reshape([seq_dim, vocab_dim]);
                    let last_token_logits = logits.slice([seq_dim - 1..seq_dim]);
                    
                    let mut logits_vec: Vec<f32> = last_token_logits.to_data().to_vec().unwrap();
                    let temperature = 0.7;
                    
                    for logit in logits_vec.iter_mut() {
                        *logit = *logit / temperature;
                    }
                    
                    let max_logit = logits_vec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                    let exp_sum: f32 = logits_vec.iter()
                        .map(|&x| (x - max_logit).exp())
                        .sum();
                    let mut probs: Vec<f32> = logits_vec.iter()
                        .map(|&x| ((x - max_logit).exp() / exp_sum))
                        .collect();
                    
                    let next_token_id = probs.iter()
                        .enumerate()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .map(|(idx, _)| idx as i32)
                        .unwrap_or(0);
                    
                    current_tokens.push(next_token_id);
                    let next_char = dataset.tokenizer.decode(&[next_token_id]);
                    
                    if next_char == "\n" {
                        break;
                    }
                    
                    response.push_str(&next_char);
                }
                
                println!("🤖 AI: {}", response);
                conversation.push_str(&response);
                conversation.push_str("\n");
            }
        }
    }
}