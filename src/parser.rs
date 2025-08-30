pub enum Command {
    ECHO(String, String),
    GET(String),
    PING,
    SET(String, String, Option<u64>),
    ERROR,
}

pub fn parse(command: String) -> Command {
    let tokens: Vec<&str> = command.split("\r\n").collect();
    if tokens.len() < 3 { 
        return Command::ERROR;
    }
    // println!("Token size:{}", tokens.len());

    match tokens[2].to_uppercase().as_str() {
        "ECHO" => Command::ECHO(tokens[3].to_string(), tokens[4].to_string()),
        "GET" => Command::GET(tokens[4].to_string()),
        "PING" => Command::PING,
        "SET" => match tokens.len() {
            8 => Command::SET(tokens[4].to_string(), tokens[6].to_string(), None),
            12 => Command::SET(tokens[4].to_string(), tokens[6].to_string(), Some(tokens[10].parse::<u64>().unwrap())),
            _ => Command::ERROR,
        },
        _ => Command::ERROR,
    }
}