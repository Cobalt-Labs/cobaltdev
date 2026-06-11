use tracing::{info, warn, error};

pub fn init_logger() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();
}

pub fn log_info(msg: &str) {
    info!("{}", msg);
}

pub fn log_warn(msg: &str) {
    warn!("{}", msg);
}

pub fn log_error(msg: &str) {
    error!("{}", msg);
}

pub fn log_agent_prompt(agent_name: &str, prompt: &str) {
    info!(target: "agent_prompt", agent = agent_name, prompt = prompt, "Sending prompt to agent");
}

pub fn log_agent_response(agent_name: &str, response: &str) {
    info!(target: "agent_response", agent = agent_name, response_length = response.len(), "Received response from agent");
}
