pub const FIRST_CLIENT_ID: usize = 1;
pub const FIRST_PLAYER_ID: usize = 0;
pub const FIRST_EGG_ID: usize = 0;
pub const FIRST_INCANTATION_ID: usize = 0;
pub const ID_INCREMENT: usize = 1;

pub const READ_BUFFER_SIZE: usize = 4096;
pub const MAX_PENDING_COMMANDS: usize = 10;
pub const ERROR_EXIT: i32 = 84;
pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

pub const WELCOME_MESSAGE: &str = "WELCOME\n";
pub const OK_RESPONSE: &str = "ok\n";
pub const KO_RESPONSE: &str = "ko\n";
pub const DEAD_RESPONSE: &str = "dead\n";
pub const ELEVATION_UNDERWAY_RESPONSE: &str = "Elevation underway\n";
pub const GRAPHIC_TEAM_NAME: &str = "GRAPHIC";
pub const USAGE: &str =
    "USAGE: ./zappy_server -p port -x width -y height -n name1 name2 ... -c clientsNb -f freq";

pub const MIN_SERVER_PORT: u16 = 1024;
pub const MIN_MAP_DIMENSION: usize = 10;
pub const MAX_MAP_DIMENSION: usize = 42;
pub const MAX_CLIENTS_PER_TEAM: usize = 200;
pub const MAX_FREQUENCY: usize = 10_000;

pub const DEFAULT_FREQUENCY: usize = 100;
pub const INITIAL_LIFE_UNITS: usize = 10;
pub const INITIAL_VISIBLE_FOOD: usize = INITIAL_LIFE_UNITS - 1;
pub const INITIAL_PLAYER_LEVEL: usize = 1;
pub const MAX_PLAYER_LEVEL: usize = 8;
pub const WINNING_PLAYER_COUNT: usize = 6;

pub const DEFAULT_COMMAND_TIME_UNITS: u64 = 7;
pub const INVENTORY_COMMAND_TIME_UNITS: u64 = 1;
pub const ZERO_COMMAND_TIME_UNITS: u64 = 0;
pub const FORK_TIME_UNITS: u64 = 42;
pub const INCANTATION_TIME_UNITS: u64 = 300;
pub const FOOD_LIFETIME_TIME_UNITS: u64 = 126;
pub const RESOURCE_RESPAWN_TIME_UNITS: u64 = 20;

pub const FOOD_DENSITY: f64 = 0.50;
pub const LINEMATE_DENSITY: f64 = 0.30;
pub const DERAUMERE_DENSITY: f64 = 0.15;
pub const SIBUR_DENSITY: f64 = 0.10;
pub const MENDIANE_DENSITY: f64 = 0.10;
pub const PHIRAS_DENSITY: f64 = 0.08;
pub const THYSTAME_DENSITY: f64 = 0.05;

pub const COMMAND_FORWARD: &str = "Forward";
pub const COMMAND_RIGHT: &str = "Right";
pub const COMMAND_LEFT: &str = "Left";
pub const COMMAND_LOOK: &str = "Look";
pub const COMMAND_INVENTORY: &str = "Inventory";
pub const COMMAND_BROADCAST: &str = "Broadcast";
pub const COMMAND_CONNECT_NBR: &str = "Connect_nbr";
pub const COMMAND_FORK: &str = "Fork";
pub const COMMAND_EJECT: &str = "Eject";
pub const COMMAND_TAKE: &str = "Take";
pub const COMMAND_SET: &str = "Set";
pub const COMMAND_INCANTATION: &str = "Incantation";
