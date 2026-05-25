wit_bindgen::generate!({
    path: "../../wit",
    world: "cube-world",
});

use serde::{Deserialize, Serialize};

use crate::local::zappy::{
    graphic::{CameraCmd, Color, CubeCmd, Grid3dCmd, Vec3},
    host_api::emit_event,
};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct SerializableCameraState {
    position: (f32, f32, f32),
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
    speed: f32,
}

struct CameraState {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
    speed: f32,
    velocity: Vec3,
    wish_dir: Vec3,
}

static CAMERA: Mutex<CameraState> = Mutex::new(CameraState {
    position: Vec3 {
        x: 0.0,
        y: 10.0,
        z: 0.0,
    },
    yaw: 0.0f32.to_radians(),
    pitch: -45.0f32.to_radians(),
    sensitivity: 0.5,
    speed: 0.2,
    velocity: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    wish_dir: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
});
static MAP_DIMENSIONS: (u32, u32) = (12, 12);

const MAX_PITCH: f32 = 89.0f32.to_radians();
const FRICTION: f32 = 8.0;
const ACCELERATION: f32 = 12.0;

struct Module;

fn build_procedural_item(tx: f32, tz: f32, idx: u32, color: Color) -> RenderCommand {
    let radius = 0.28_f32;
    let golden_angle = 2.39996_f32;
    let angle = (idx as f32) * golden_angle;

    let final_position = Vec3 {
        x: tx + (radius * angle.cos()),
        y: 0.12_f32,
        z: tz + (radius * angle.sin()),
    };

    RenderCommand::Cube(CubeCmd {
        position: final_position,
        size: Vec3 {
            x: 0.12,
            y: 0.12,
            z: 0.12,
        },
        color,
    })
}

impl Guest for Module {
    fn serialize() -> Vec<u8> {
        let camera = CAMERA.lock().unwrap();
        let serialize = SerializableCameraState {
            position: (camera.position.x, camera.position.y, camera.position.z),
            yaw: camera.yaw,
            pitch: camera.pitch,
            sensitivity: camera.sensitivity,
            speed: camera.speed,
        };
        bincode::serialize(&serialize).unwrap_or_default()
    }

    fn deserialize(state_bytes: Vec<u8>) {
        if let Ok(decoded) = bincode::deserialize::<SerializableCameraState>(&state_bytes) {
            let mut state = CAMERA.lock().unwrap();
            *state = CameraState {
                position: Vec3 {
                    x: decoded.position.0,
                    y: decoded.position.1,
                    z: decoded.position.2,
                },
                yaw: decoded.yaw,
                pitch: decoded.pitch,
                sensitivity: decoded.sensitivity,
                speed: decoded.speed,
                velocity: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                wish_dir: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            };
        }
    }

    fn handle_input(state: InputState) {
        let mut c = CAMERA.lock().unwrap();

        c.yaw += state.mouse_delta.x * c.sensitivity;
        c.yaw %= f32::to_radians(360.0);
        c.pitch += state.mouse_delta.y * c.sensitivity;
        c.pitch = c.pitch.clamp(-MAX_PITCH, MAX_PITCH);

        let (fx, fy, fz) = (
            c.yaw.sin() * c.pitch.cos(),
            c.pitch.sin(),
            c.yaw.cos() * c.pitch.cos(),
        );
        let (sx, sz) = (-fz, fx);

        let mut wd = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for action in state.actions {
            match action {
                InputAction::MoveForward => {
                    wd.x += fx;
                    wd.y += fy;
                    wd.z += fz;
                }
                InputAction::MoveBackward => {
                    wd.x -= fx;
                    wd.y -= fy;
                    wd.z -= fz;
                }
                InputAction::MoveLeft => {
                    wd.x -= sx;
                    wd.z -= sz;
                }
                InputAction::MoveRight => {
                    wd.x += sx;
                    wd.z += sz;
                }
                InputAction::ScrollUp => {
                    c.speed *= 1.20;
                    c.speed = c.speed.clamp(0.1, 10.0);
                }
                InputAction::ScrollDown => {
                    c.speed /= 1.20;
                    c.speed = c.speed.clamp(0.1, 10.0);
                }
                _ => {}
            }
        }

        let len = (wd.x * wd.x + wd.y * wd.y + wd.z * wd.z).sqrt();
        if len > 1.0 {
            wd.x /= len;
            wd.y /= len;
            wd.z /= len;
        }
        c.wish_dir = wd;
    }

    fn update_module(_time: f32, dt: f32, _w: f32, _h: f32) -> Vec<RenderCommand> {
        let mut cmds = Vec::new();
        let mut camera = CAMERA.lock().unwrap();

        camera.velocity.x -= camera.velocity.x * FRICTION * dt;
        camera.velocity.y -= camera.velocity.y * FRICTION * dt;
        camera.velocity.z -= camera.velocity.z * FRICTION * dt;

        camera.velocity.x += camera.wish_dir.x * camera.speed * ACCELERATION * dt;
        camera.velocity.y += camera.wish_dir.y * camera.speed * ACCELERATION * dt;
        camera.velocity.z += camera.wish_dir.z * camera.speed * ACCELERATION * dt;

        camera.position.x += camera.velocity.x * dt;
        camera.position.y += camera.velocity.y * dt;
        camera.position.z += camera.velocity.z * dt;

        let target = Vec3 {
            x: camera.position.x + camera.yaw.sin() * camera.pitch.cos(),
            y: camera.position.y + camera.pitch.sin(),
            z: camera.position.z + camera.yaw.cos() * camera.pitch.cos(),
        };

        cmds.push(RenderCommand::Camera(CameraCmd {
            position: camera.position,
            target,
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            fovy: 70.0,
        }));

        cmds.push(RenderCommand::Grid3d(Grid3dCmd {
            slices: MAP_DIMENSIONS.0 * 2,
            spacing: 0.5,
            color1: Color {
                r: 60,
                g: 60,
                b: 70,
                a: 255,
            },
            color2: Color {
                r: 60,
                g: 100,
                b: 190,
                a: 255,
            },
        }));

        for x in 0..MAP_DIMENSIONS.0 {
            for z in 0..MAP_DIMENSIONS.1 {
                let (fx, fz) = (x as f32 - 5.5, z as f32 - 5.5);
                for idx in 0..4 {
                    let color = match idx {
                        0 => Color {
                            r: 255,
                            g: 80,
                            b: 80,
                            a: 255,
                        },
                        1 => Color {
                            r: 80,
                            g: 255,
                            b: 150,
                            a: 255,
                        },
                        2 => Color {
                            r: 80,
                            g: 150,
                            b: 255,
                            a: 255,
                        },
                        _ => Color {
                            r: 200,
                            g: 200,
                            b: 80,
                            a: 255,
                        },
                    };
                    cmds.push(build_procedural_item(fx, fz, idx, color));
                }
            }
        }

        emit_event(
            "overlay:update_metric",
            &format!("Boxes:{}", MAP_DIMENSIONS.0 * MAP_DIMENSIONS.1),
        );
        emit_event(
            "overlay:update_metric",
            &format!(
                "Pitch:{:.2}° Yaw: {:.2}°",
                camera.pitch.to_degrees() + 89.0,
                camera.yaw.to_degrees().abs()
            ),
        );
        emit_event(
            "overlay:update_metric",
            &format!(
                "X:{:.2} Y: {:.2} Z: {:.2}",
                camera.position.x, camera.position.y, camera.position.z
            ),
        );

        cmds
    }

    fn get_commands() -> Vec<CommandDesc> {
        Vec::new()
    }
    fn run_command(_cmd: String, _args: Vec<String>) -> ResponseCommand {
        ResponseCommand::Unknown
    }
    fn handle_event(_name: String, _payload: String) {}
}

export!(Module);
