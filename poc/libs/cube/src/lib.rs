wit_bindgen::generate!({
    path: "../../wit",
    world: "cube-world",
});

use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::local::zappy::graphic::{Color, RectCmd};

#[derive(Serialize, Deserialize, Default)]
struct CubeData {
    offsets: (f32, f32),
    clockwise: bool,
}

static CUBE_STATE: Mutex<CubeData> = Mutex::new(CubeData {
    offsets: (200.0, 200.0),
    clockwise: false,
});

struct Module;

impl Guest for Module {
    fn serialize() -> Vec<u8> {
        let state = CUBE_STATE.lock().unwrap();
        bincode::serialize(&*state).unwrap_or_default()
    }

    fn deserialize(state_bytes: Vec<u8>) {
        if let Ok(decoded) = bincode::deserialize::<CubeData>(&state_bytes) {
            let mut state = CUBE_STATE.lock().unwrap();
            *state = decoded;
        }
    }

    fn get_commands() -> Vec<CommandDesc> {
        vec![CommandDesc {
            module: "cube".to_string(),
            name: "invert_cube".to_string(),
            options: "".to_string(),
            help: "Invert cube direction".to_string(),
        }]
    }

    fn run_command(cmd: String, _args: Vec<String>) -> ResponseCommand {
        match cmd.as_str() {
            "invert_cube" => {
                let mut state = CUBE_STATE.lock().unwrap();
                state.clockwise = !state.clockwise;
                ResponseCommand::Ok
            }
            _ => ResponseCommand::Unknown,
        }
    }

    fn handle_input(state: InputState) {
        if matches!(state.context, EngineContext::Gameplay) {
            let mut data = CUBE_STATE.lock().unwrap();

            for action in state.actions {
                match action {
                    InputAction::MoveLeft => data.offsets.0 -= 10.0,
                    InputAction::MoveRight => data.offsets.0 += 10.0,
                    InputAction::MoveUp => data.offsets.1 -= 10.0,
                    InputAction::MoveDown => data.offsets.1 += 10.0,
                    _ => {}
                }
            }
        }
    }

    fn update_module(time: f32, _dt: f32, _w: f32, _h: f32) -> Vec<RenderCommand> {
        let data = CUBE_STATE.lock().unwrap();
        let speed = 2.0;
        let time = if data.clockwise { time } else { -time };
        let angle = time * speed;

        vec![
            RenderCommand::Rect(RectCmd {
                x: angle.cos() * 100.0 + data.offsets.0,
                y: angle.sin() * 100.0 + data.offsets.1,
                w: 25.0,
                h: 25.0,
                rotation: angle,
                color: Color {
                    r: 135,
                    g: 206,
                    b: 235,
                    a: 255,
                },
            }),
            RenderCommand::Rect(RectCmd {
                x: data.offsets.0,
                y: data.offsets.1,
                w: 5.0,
                h: 5.0,
                color: Color {
                    r: 135,
                    g: 206,
                    b: 235,
                    a: 255,
                },
                rotation: 0.0,
            }),
        ]
    }
}

export!(Module);
