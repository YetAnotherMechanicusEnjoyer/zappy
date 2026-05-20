use crate::plugin::{CubeState, KeyEvent, PluginInstance};
use colored::*;
use macroquad::prelude::*;
use std::path::{Path, PathBuf};
use wasmtime::Engine;

pub struct PluginManager {
    engine: Engine,
    pipeline: Vec<PluginInstance>,
}

impl PluginManager {
    pub fn new(engine: Engine) -> Self {
        let mut manager = Self {
            engine,
            pipeline: Vec::new(),
        };
        manager.scan_and_load_all();
        manager
    }

    pub fn scan_and_load_all(&mut self) {
        self.pipeline.clear();
        let dir = Path::new("plugins");
        if !dir.exists() {
            return;
        }

        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("wasm"))
            .collect();

        entries.sort();

        for path in entries {
            if let Ok(plugin) = PluginInstance::load(&self.engine, &path) {
                self.pipeline.push(plugin)
            }
        }
    }

    pub fn reload_plugin(&mut self, plugin_name: &str) {
        let path = format!("plugins/{}.wasm", plugin_name);

        self.pipeline.retain(|p| p.name != plugin_name);

        match PluginInstance::load(&self.engine, Path::new(&path)) {
            Ok(new_plugin) => {
                self.pipeline.push(new_plugin);
                self.pipeline.sort_by(|a, b| a.name.cmp(&b.name));
            }
            Err(e) => eprintln!(
                "{} {} {} {e}",
                "[WATCHER]".yellow().bold(),
                plugin_name.italic(),
                "is compiling or got an error:".bright_black()
            ),
        }
    }

    pub fn handle_inputs(&mut self, event: KeyEvent) {
        let mut to_remove = Vec::new();
        let mut input_blocked = false;

        for (index, plugin) in self.pipeline.iter_mut().enumerate() {
            if input_blocked {
                break;
            }

            match plugin.bindings.call_handle_input(&mut plugin.store, &event) {
                Ok(consumed) => {
                    if consumed {
                        /*println!(
                            "{}{} {} {}",
                            "↳".bright_black(),
                            "[INPUT]".bright_magenta().bold(),
                            "blocked by".bright_black(),
                            plugin.name.bright_black().underline()
                        );*/
                        input_blocked = true;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} {} {} {} {e}",
                        "[CRASH]".red().bold(),
                        "Plugin".bright_black(),
                        plugin.name.italic(),
                        "panicked (Input):".bright_black()
                    );
                    to_remove.push(index);
                }
            }
        }

        for index in to_remove.into_iter().rev() {
            self.pipeline.remove(index);
        }
    }

    pub fn update_and_render(&mut self, state: CubeState) {
        self.pipeline.retain_mut(|plugin| {
            match plugin.bindings.call_update_cube(&mut plugin.store, state) {
                Ok(cmd) => {
                    if cmd.x != 0.0 || cmd.y != 0.0 {
                        draw_rectangle_ex(
                            (screen_width() / 2.0) + cmd.x - 25.0,
                            (screen_height() / 2.0) + cmd.y - 25.0,
                            50.0,
                            50.0,
                            DrawRectangleParams {
                                rotation: cmd.rotation,
                                color: SKYBLUE,
                                ..Default::default()
                            },
                        );
                    }
                    true
                }
                Err(e) => {
                    eprintln!(
                        "{} {} {} {} {e}",
                        "[CRASH]".red().bold(),
                        "Plugin".bright_black(),
                        plugin.name.italic(),
                        "panicked (Update):".bright_black()
                    );
                    false
                }
            }
        });
    }
}
