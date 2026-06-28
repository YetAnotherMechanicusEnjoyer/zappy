use crate::constants::{
    DEFAULT_FREQUENCY, GRAPHIC_TEAM_NAME, MAX_CLIENTS_PER_TEAM, MAX_FREQUENCY, MAX_MAP_DIMENSION,
    MIN_MAP_DIMENSION, MIN_SERVER_PORT,
};
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub width: usize,
    pub height: usize,
    pub teams: Vec<String>,
    pub clients_nb: usize,
    pub freq: usize,
}

pub enum ParseOutcome {
    Config(Config),
    Help,
}

trait EnvFallback: Sized {
    fn has_value(&self) -> bool;

    fn parse_from_env(s: &str) -> Option<Self>;
}

impl<T: FromStr> EnvFallback for Option<T> {
    fn has_value(&self) -> bool {
        self.is_some()
    }

    fn parse_from_env(s: &str) -> Option<Self> {
        s.parse::<T>().ok().map(Some)
    }
}

impl<T: FromStr> EnvFallback for Vec<T> {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }

    fn parse_from_env(s: &str) -> Option<Self> {
        if s.trim().is_empty() {
            return Some(Vec::new());
        }

        let items: Result<Vec<T>, _> = s.split(',').map(|val| val.trim().parse::<T>()).collect();

        items.ok()
    }
}

pub fn parse_args() -> Result<ParseOutcome, String> {
    parse_arguments(env::args().skip(1).collect())
}

fn default_env_variable<T>(var: T, env_var: &str) -> T
where
    T: EnvFallback,
{
    if var.has_value() {
        return var;
    }

    if let Ok(s) = std::env::var(env_var) {
        if let Some(parsed) = T::parse_from_env(&s) {
            return parsed;
        }
    }

    var
}

fn parse_arguments(arguments: Vec<String>) -> Result<ParseOutcome, String> {
    if arguments.iter().any(|argument| argument == "--help") {
        return Ok(ParseOutcome::Help);
    }

    let mut port = None;
    let mut width = None;
    let mut height = None;
    let mut teams = Vec::new();
    let mut clients_nb = None;
    let mut frequency = Some(DEFAULT_FREQUENCY);
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "-p" => port = Some(parse_next::<u16>(&arguments, &mut index, "port")?),
            "-x" => width = Some(parse_next::<usize>(&arguments, &mut index, "width")?),
            "-y" => height = Some(parse_next::<usize>(&arguments, &mut index, "height")?),
            "-c" => clients_nb = Some(parse_next::<usize>(&arguments, &mut index, "clientsNb")?),
            "-f" => frequency = Some(parse_next::<usize>(&arguments, &mut index, "freq")?),
            "-n" => {
                index += 1;
                while index < arguments.len() && !arguments[index].starts_with('-') {
                    teams.push(arguments[index].clone());
                    index += 1;
                }
                continue;
            }
            unknown => return Err(format!("unknown option: {unknown}")),
        }
        index += 1;
    }

    port = default_env_variable(port, "ZAPPY_PORT");
    width = default_env_variable(width, "ZAPPY_WIDTH");
    height = default_env_variable(height, "ZAPPY_HEIGHT");
    teams = default_env_variable(teams, "ZAPPY_TEAMS");
    clients_nb = default_env_variable(clients_nb, "ZAPPY_NB_CLIENTS");
    frequency = default_env_variable(frequency, "ZAPPY_FREQUENCY");

    let config = Config {
        port: required(port, "port")?,
        width: required(width, "width")?,
        height: required(height, "height")?,
        teams,
        clients_nb: required(clients_nb, "clientsNb")?,
        freq: required(frequency, "freq")?,
    };
    validate_config(&config)?;
    Ok(ParseOutcome::Config(config))
}

fn parse_next<T>(arguments: &[String], index: &mut usize, name: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    *index += 1;
    let value = arguments
        .get(*index)
        .ok_or_else(|| format!("missing value for {name}"))?;
    value
        .parse::<T>()
        .map_err(|_| format!("invalid value for {name}: {value}"))
}

fn required<T>(value: Option<T>, name: &str) -> Result<T, String> {
    value.ok_or_else(|| format!("missing required option: {name}"))
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.port < MIN_SERVER_PORT {
        return Err(format!(
            "port must be between {MIN_SERVER_PORT} and {}",
            u16::MAX
        ));
    }
    if !(MIN_MAP_DIMENSION..=MAX_MAP_DIMENSION).contains(&config.width)
        || !(MIN_MAP_DIMENSION..=MAX_MAP_DIMENSION).contains(&config.height)
    {
        return Err(format!(
            "map dimensions must be between {MIN_MAP_DIMENSION} and {MAX_MAP_DIMENSION}"
        ));
    }
    if !(1..=MAX_CLIENTS_PER_TEAM).contains(&config.clients_nb) {
        return Err(format!(
            "clientsNb must be between 1 and {MAX_CLIENTS_PER_TEAM}"
        ));
    }
    if !(1..=MAX_FREQUENCY).contains(&config.freq) {
        return Err(format!("freq must be between 1 and {MAX_FREQUENCY}"));
    }
    if config.teams.is_empty() {
        return Err("at least one team name is required".to_string());
    }
    if config.teams.iter().any(|team| team == GRAPHIC_TEAM_NAME) {
        return Err("GRAPHIC is a reserved team name".to_string());
    }
    if config.teams.iter().any(|team| team.is_empty()) {
        return Err("team names cannot be empty".to_string());
    }
    let unique = config.teams.iter().collect::<HashSet<_>>();
    if unique.len() != config.teams.len() {
        return Err("team names must be unique".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_config, Config};
    use crate::constants::{
        DEFAULT_FREQUENCY, MAX_MAP_DIMENSION, MIN_MAP_DIMENSION, MIN_SERVER_PORT,
    };

    fn valid_config() -> Config {
        Config {
            port: MIN_SERVER_PORT,
            width: MIN_MAP_DIMENSION,
            height: MIN_MAP_DIMENSION,
            teams: vec!["team1".to_string(), "team2".to_string()],
            clients_nb: 2,
            freq: DEFAULT_FREQUENCY,
        }
    }

    #[test]
    fn accepts_reference_limits() {
        let mut config = valid_config();
        config.width = MAX_MAP_DIMENSION;
        config.height = MAX_MAP_DIMENSION;
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn rejects_reserved_team() {
        let mut config = valid_config();
        config.teams = vec!["GRAPHIC".to_string()];
        assert!(validate_config(&config).is_err());
    }
}
