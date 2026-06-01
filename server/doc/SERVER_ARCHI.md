# Zappy Server Architecture

## 1. Goal of the server

The `zappy_server` is the central authority of the Zappy project.

It is responsible for:

- accepting AI and GUI clients
- owning the game world
- storing teams, players, eggs, resources and tiles
- receiving commands from clients
- executing commands after their Zappy time delay
- sending responses to AI clients
- sending world updates to GUI clients
- detecting player death
- detecting the winning team

The server must run as a single process and a single thread. It must use `poll` to handle several sockets without blocking.

## 2. Current state

Current implemented features:

- Rust server crate created
- command-line argument parsing
- TCP listener creation
- `mio` poll loop
- client accept handling
- `WELCOME\n` message sent to new clients
- basic client read handling
- named constants used instead of magic numbers

Not implemented yet:

- real Zappy handshake
- AI team authentication
- GUI authentication with `GRAPHIC`
- player creation
- map generation
- resources
- command queue
- timed events
- GUI protocol
- death system
- incantation
- win condition

## 3. Project constraints

The server binary must be named:

```txt
zappy_server
````

Expected usage:

```txt
./zappy_server -p port -x width -y height -n name1 name2 ... -c clientsNb -f freq
```

Options:

```txt
-p port       server port
-x width      world width
-y height     world height
-n names      team names
-c clientsNb  authorized clients per team
-f freq       reciprocal of time unit
```

The reserved team name for the graphical client is:

```txt
GRAPHIC
```

AI clients connect by TCP. The server first sends:

```txt
WELCOME\n
```

Then the client sends its team name.

For an AI client, the server must answer:

```txt
CLIENT-NUM\n
X Y\n
```

Where:

- `CLIENT-NUM` is the number of available slots for the team
- `X Y` is the size of the world

## 4. Server mental model

The server is split into layers.

```txt
Network layer
    accepts clients
    reads bytes from sockets
    writes queued responses

Protocol layer
    understands AI commands
    understands GUI commands
    formats responses

Game layer
    owns map, players, teams, eggs and resources
    applies game rules

Time layer
    schedules commands
    executes actions only when their delay is finished

GUI notification layer
    sends world updates to graphical clients
```

The rule is:

```txt
network code should not directly implement game logic
game logic should not directly manage sockets
```

## 5. Current main functions

### `main`

Entry point of the server.

Responsibilities:

- parse command-line arguments
- create the poll instance
- create the TCP listener
- register the listener in poll
- store connected clients
- run the main event loop

Current flow:

```txt
parse args
create listener
register listener into poll
loop:
    wait for socket activity
    if server socket is ready:
        accept new clients
    if client socket is ready:
        read client data
```

### `parse_args`

Reads the command-line arguments and builds a `Config`.

Example accepted command:

```bash
./zappy_server -p 4242 -x 10 -y 10 -n team1 team2 -c 3 -f 100
```

Returns:

```rust
Config {
    port,
    width,
    height,
    teams,
    clients_nb,
    freq,
}
```

### `validate_config`

Checks if the parsed configuration is valid.

Current checks:

- port must be greater than 0
- width must be greater than 0
- height must be greater than 0
- at least one team must exist
- clients number must be greater than 0
- frequency must be greater than 0

### `create_listener`

Creates the TCP listener.

The listener is the socket that waits for new clients.

### `accept_new_clients`

Accepts every pending client connection.

Current behavior:

- accepts the client
- gives it a unique poll token
- registers the client socket into poll
- sends `WELCOME\n`
- stores the client in the clients map

### `read_from_client`

Reads data from a connected client.

Current behavior:

- reads bytes from the socket
- prints the received message
- removes the client if it disconnects

Later, this function should not execute commands directly.

Instead, it should:

```txt
read bytes
add bytes to client input buffer
extract complete lines ending with '\n'
send each complete command line to the protocol layer
```

## 6. Important constants

This project follows the rule:

```txt
no magic numbers
```

A magic number is a value written directly in code without a name.

Bad:

```rust
let mut buffer = [0; 512];
std::process::exit(84);
```

Good:

```rust
const READ_BUFFER_SIZE: usize = 512;
const EPITECH_ERROR_EXIT: i32 = 84;

let mut buffer = [0; READ_BUFFER_SIZE];
std::process::exit(EPITECH_ERROR_EXIT);
```

Current constants:

```rust
SERVER_TOKEN_ID
FIRST_CLIENT_TOKEN_ID
TOKEN_INCREMENT
EVENTS_CAPACITY
READ_BUFFER_SIZE
EPITECH_ERROR_EXIT
DEFAULT_BIND_ADDRESS
WELCOME_MESSAGE
HELP_FLAG
PORT_FLAG
WIDTH_FLAG
HEIGHT_FLAG
TEAM_NAMES_FLAG
CLIENTS_NB_FLAG
FREQUENCY_FLAG
USAGE
MIN_PORT
MIN_WIDTH
MIN_HEIGHT
MIN_CLIENTS_NB
MIN_FREQUENCY
```

Future constants to add:

```rust
MAX_PENDING_COMMANDS
FOOD_LIFE_TIME_UNITS
INITIAL_FOOD_UNITS
RESOURCE_RESPAWN_TIME
FORWARD_TIME
RIGHT_TIME
LEFT_TIME
LOOK_TIME
INVENTORY_TIME
BROADCAST_TIME
FORK_TIME
EJECT_TIME
TAKE_TIME
SET_TIME
INCANTATION_TIME
```

## 7. How to add a new server feature

When adding a feature, follow this order.

### Step 1: define the data

Ask:

```txt
What state does this feature need?
Where should this state live?
```

Example:

For player movement, we need:

```rust
Player {
    id,
    x,
    y,
    orientation,
    level,
    inventory,
}
```

### Step 2: define constants

No magic numbers.

Example:

```rust
const FORWARD_TIME: u64 = 7;
```

### Step 3: parse the command

The protocol layer should recognize the command string.

Example:

```txt
Forward
```

### Step 4: schedule the action

Most AI commands are not instant.

Example:

```txt
Forward takes 7 / freq seconds
```

So the server should create a timed event instead of moving immediately.

### Step 5: execute the action

When the event is ready, the game layer applies the real effect.

Example:

```txt
update player position
send "ok\n" to AI
send "ppo #n X Y O\n" to GUI
```

### Step 6: test manually

Use `nc` to test the server.

Example:

```bash
nc localhost 4242
```

### Step 7: update this documentation

Every feature should add:

- what was added
- which files were changed
- important functions
- how to test it
- known limitations

## 8. Feature documentation template

Use this template when adding a new feature.

````md
## Feature: FEATURE_NAME

### Goal

Explain what the feature does.

### Files changed

- `path/to/file.rs`
- `path/to/other_file.rs`

### New constants

```rust
const EXAMPLE_CONST: usize = 10;
````

### New structs / enums

```rust
struct Example {
    field: usize,
}
```

### Main functions

#### `function_name`

Explain what the function does.

### Flow

```txt
client sends command
server parses command
server schedules event
event executes
server sends response
GUI is notified if needed
```

### Manual test

```bash
cargo run -- -p 4242 -x 10 -y 10 -n team1 team2 -c 3 -f 100
nc localhost 4242
```

### Expected result

```txt
WELCOME
```

### Notes

Add warnings, limitations or future improvements here.

````

## 9. Next planned feature

Next feature:

```txt
real Zappy handshake
````

Goal:

```txt
client connects
server sends WELCOME
client sends either:
    GRAPHIC
    or a team name

if GRAPHIC:
    client becomes GUI

if valid team name:
    client becomes AI/player
    server sends:
        CLIENT-NUM
        X Y

if invalid team name:
    server disconnects or answers ko depending on chosen handling
```

This will require:

- a client state enum
- an input buffer per client
- line extraction
- team validation
- client type detection

````

Then commit it separately:

```bash
git add server/docs/SERVER_ARCHITECTURE.md
git commit -m "Add server architecture documentation" -m "Document the current Rust server foundation, main functions, project constraints, no magic number rule, and the template to follow when adding new server features."
````

This is a good doc commit because it is not mixed with code. It explains your architecture and gives your teammate a guide for future features.

The parts about `zappy_server` usage, `GRAPHIC`, `WELCOME`, AI handshake, single process/thread, and `poll` come directly from the Zappy subject.
