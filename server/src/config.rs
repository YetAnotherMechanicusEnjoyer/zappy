use crate::constants::{
    CLIENTS_NB_FLAG, FREQUENCY_FLAG, HEIGHT_FLAG, HELP_FLAG, MIN_CLIENTS_NB, MIN_FREQUENCY,
    MIN_HEIGHT, MIN_PORT, MIN_WIDTH, PORT_FLAG, TEAM_NAMES_FLAG, USAGE, WIDTH_FLAG,
};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub width: usize,
    pub height: usize,
    pub teams: Vec<String>,
    pub clients_nb: usize,
    pub freq: usize,
}

pub fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == HELP_FLAG) {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    let mut port = None;
    let mut width = None;
    let mut height = None;
    let mut teams = Vec::new();
    let mut clients_nb = None;
    let mut freq = None;

    let mut args_iter = args.iter().skip(1).peekable();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            PORT_FLAG => {
                port = Some(parse_next_value::<u16>(&mut args_iter, PORT_FLAG)?);
            }
            WIDTH_FLAG => {
                width = Some(parse_next_value::<usize>(&mut args_iter, WIDTH_FLAG)?);
            }
            HEIGHT_FLAG => {
                height = Some(parse_next_value::<usize>(&mut args_iter, HEIGHT_FLAG)?);
            }
            CLIENTS_NB_FLAG => {
                clients_nb = Some(parse_next_value::<usize>(&mut args_iter, CLIENTS_NB_FLAG)?);
            }
            FREQUENCY_FLAG => {
                freq = Some(parse_next_value::<usize>(&mut args_iter, FREQUENCY_FLAG)?);
            }
            TEAM_NAMES_FLAG => {
                parse_team_names(&mut args_iter, &mut teams);
            }
            _ => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }

    let config = Config {
        port: port.ok_or("Missing -p port")?,
        width: width.ok_or("Missing -x width")?,
        height: height.ok_or("Missing -y height")?,
        teams,
        clients_nb: clients_nb.ok_or("Missing -c clientsNb")?,
        freq: freq.ok_or("Missing -f freq")?,
    };

    validate_config(&config)?;
    Ok(config)
}

fn parse_next_value<'a, T>(
    args_iter: &mut std::iter::Peekable<std::iter::Skip<std::slice::Iter<'a, String>>>,
    flag: &str,
) -> Result<T, String>
where
    T: std::str::FromStr,
{
    let value = args_iter
        .next()
        .ok_or_else(|| format!("Missing value for {}", flag))?;

    value
        .parse::<T>()
        .map_err(|_| format!("Invalid value for {}", flag))
}

fn parse_team_names<'a>(
    args_iter: &mut std::iter::Peekable<std::iter::Skip<std::slice::Iter<'a, String>>>,
    teams: &mut Vec<String>,
) {
    while let Some(next_arg) = args_iter.peek() {
        if next_arg.starts_with('-') {
            break;
        }

        if let Some(team_name) = args_iter.next() {
            teams.push(team_name.clone());
        }
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.port < MIN_PORT {
        return Err("Port must be greater than 0".to_string());
    }

    if config.width < MIN_WIDTH {
        return Err("Width must be greater than 0".to_string());
    }

    if config.height < MIN_HEIGHT {
        return Err("Height must be greater than 0".to_string());
    }

    if config.teams.is_empty() {
        return Err("Missing teams after -n".to_string());
    }

    if config.clients_nb < MIN_CLIENTS_NB {
        return Err("clientsNb must be greater than 0".to_string());
    }

    if config.freq < MIN_FREQUENCY {
        return Err("freq must be greater than 0".to_string());
    }

    Ok(())
}
