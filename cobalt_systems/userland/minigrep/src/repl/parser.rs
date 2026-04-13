pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_input(input: &str) -> Option<Command> {
    let parts: Vec<String> = input.split_whitespace().map(|x| x.into()).collect();
    if parts.is_empty() {
        return None;
    }

    Some(Command {
        name: parts[0].clone(),
        args: parts[1..].to_vec(),
    })
}
