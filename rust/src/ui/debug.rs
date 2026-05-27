use godot::classes::{CanvasLayer, Label, Engine, Os, Performance, InputEvent, CharacterBody2D, ICanvasLayer, performance};
use godot::prelude::*;

const INDENT_LABEL: &str = "   ";

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct DebugUI {
    // Labels
    error_panel: Option<Gd<Label>>,
    
    // System Labels
    fps_label: Option<Gd<Label>>,
    memory_label: Option<Gd<Label>>,
    draw_calls_label: Option<Gd<Label>>,
    process_time_label: Option<Gd<Label>>,
    physics_time_label: Option<Gd<Label>>,

    // Player Labels
    pos_label: Option<Gd<Label>>,
    grid_pos_label: Option<Gd<Label>>,
    chunk_pos_label: Option<Gd<Label>>,
    state_label: Option<Gd<Label>>,
    velocity_label: Option<Gd<Label>>,
    is_flying_label: Option<Gd<Label>>,
    jumps_left_label: Option<Gd<Label>>,

    // References
    player_node: Option<Gd<CharacterBody2D>>,
    terrain_node: Option<Gd<Node2D>>,

    base: Base<CanvasLayer>,
}

#[godot_api]
impl DebugUI {
    fn ensure_references(&mut self) {
        if self.player_node.is_none() {
            let tree = self.base().get_tree();
            if let Some(node) = tree.get_first_node_in_group("player") {
                self.player_node = node.try_cast::<CharacterBody2D>().ok();
            }
        }
        if self.terrain_node.is_none() {
            let tree = self.base().get_tree();
            if let Some(node) = tree.get_first_node_in_group("terrain") {
                self.terrain_node = node.try_cast::<Node2D>().ok();
            }
        }
    }

    fn update_system_data(&mut self) {
        let engine = Engine::singleton();
        let os = Os::singleton();
        let performance = Performance::singleton();

        if let Some(mut label) = self.fps_label.clone() {
            label.set_text(&format!("{}FPS: {}", INDENT_LABEL, engine.get_frames_per_second()));
        }

        if let Some(mut label) = self.memory_label.clone() {
            let mem_mb = os.get_static_memory_usage() as f64 / 1048576.0;
            label.set_text(&format!("{}Memory: {:.2} MB", INDENT_LABEL, mem_mb));
        }

        if let Some(mut label) = self.draw_calls_label.clone() {
            let calls = performance.get_monitor(performance::Monitor::RENDER_TOTAL_DRAW_CALLS_IN_FRAME);
            label.set_text(&format!("{}Draw Calls: {}", INDENT_LABEL, calls));
        }

        if let Some(mut label) = self.process_time_label.clone() {
            let p_time = performance.get_monitor(performance::Monitor::TIME_PROCESS) * 1000.0;
            label.set_text(&format!("{}Process: {:.2} ms", INDENT_LABEL, p_time));
        }

        if let Some(mut label) = self.physics_time_label.clone() {
            let ph_time = performance.get_monitor(performance::Monitor::TIME_PHYSICS_PROCESS) * 1000.0;
            label.set_text(&format!("{}Physics: {:.2} ms", INDENT_LABEL, ph_time));
        }
    }

    fn update_player_data(&mut self) {
        let player = match &self.player_node {
            Some(p) => p,
            None => return,
        };

        let player_pos = player.get_global_position();
        if let Some(mut label) = self.pos_label.clone() {
            label.set_text(&format!("{}Pos: ({:.0}, {:.0})", INDENT_LABEL, player_pos.x, player_pos.y));
        }

        let mut cell_pos = Vector2i::ZERO;
        let mut has_terrain = false;

        if let Some(terrain) = &self.terrain_node {
            if let Some(tile_map_node) = terrain.get_node_or_null("TileMapLayer") {
                if let Ok(tile_map) = tile_map_node.try_cast::<godot::classes::TileMapLayer>() {
                    let local_pos = tile_map.to_local(player_pos);
                    cell_pos = tile_map.local_to_map(local_pos);
                    has_terrain = true;
                }
            }
        }

        if let Some(mut label) = self.grid_pos_label.clone() {
            if has_terrain {
                label.set_text(&format!("{}Cell: ({}, {})", INDENT_LABEL, cell_pos.x, cell_pos.y));
            } else {
                label.set_text(&format!("{}Cell: --", INDENT_LABEL));
            }
        }

        if let Some(mut label) = self.chunk_pos_label.clone() {
            if has_terrain {
                if let Some(terrain) = &self.terrain_node {
                    if let Some(mut gen) = terrain.get_node_or_null("TerrainGenerator") {
                        let chunk_size = gen.call("get_chunk_size", &[]).try_to::<i32>().unwrap_or(16);
                        let chunk_x = cell_pos.x.div_euclid(chunk_size);
                        let chunk_y = cell_pos.y.div_euclid(chunk_size);
                        label.set_text(&format!("{}Chunk: ({}, {})", INDENT_LABEL, chunk_x, chunk_y));
                    }
                }
            } else {
                label.set_text(&format!("{}Chunk: --", INDENT_LABEL));
            }
        }

        let player_fsm = player.get_node_or_null("PlayerFSM");

        if let Some(mut label) = self.state_label.clone() {
            if let Some(mut fsm) = player_fsm.clone() {
                let state_name = fsm.call("get_state_name", &[]).to_string();
                label.set_text(&format!("{}State: {}", INDENT_LABEL, state_name));
            } else {
                label.set_text(&format!("{}State: --", INDENT_LABEL));
            }
        }

        let velocity = player.get_velocity();
        if let Some(mut label) = self.velocity_label.clone() {
            label.set_text(&format!("{}Velocity: ({:.0}, {:.0})", INDENT_LABEL, velocity.x, velocity.y));
        }

        if let Some(mut label) = self.is_flying_label.clone() {
            let is_flying = player.get("is_flying").try_to::<bool>().unwrap_or(false);
            label.set_text(&format!("{}Is Flying: {}", INDENT_LABEL, is_flying));
        }

        if let Some(mut label) = self.jumps_left_label.clone() {
            if let Some(mut fsm) = player_fsm {
                let jumps = fsm.call("get_jumps_left", &[]).try_to::<i32>().unwrap_or(0);
                label.set_text(&format!("{}Jumps Left: {}", INDENT_LABEL, jumps));
            } else {
                label.set_text(&format!("{}Jumps Left: --", INDENT_LABEL));
            }
        }
    }
}

#[godot_api]
impl ICanvasLayer for DebugUI {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            error_panel: None,
            fps_label: None,
            memory_label: None,
            draw_calls_label: None,
            process_time_label: None,
            physics_time_label: None,
            pos_label: None,
            grid_pos_label: None,
            chunk_pos_label: None,
            state_label: None,
            velocity_label: None,
            is_flying_label: None,
            jumps_left_label: None,
            player_node: None,
            terrain_node: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_visible(false);

        // Link nodes
        self.error_panel = self.base().try_get_node_as::<Label>("MainPanel/ErrorPanel");
        
        self.fps_label = self.base().try_get_node_as::<Label>("MainPanel/SystemPanel/FPSLabel");
        self.memory_label = self.base().try_get_node_as::<Label>("MainPanel/SystemPanel/MemoryLabel");
        self.draw_calls_label = self.base().try_get_node_as::<Label>("MainPanel/SystemPanel/DrawCalls");
        self.process_time_label = self.base().try_get_node_as::<Label>("MainPanel/SystemPanel/ProcessTime");
        self.physics_time_label = self.base().try_get_node_as::<Label>("MainPanel/SystemPanel/PhysicsTime");

        self.pos_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/PosLabel");
        self.grid_pos_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/GridPosLabel");
        self.chunk_pos_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/ChunkPosLabel");
        self.state_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/StateLabel");
        self.velocity_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/VelocityLabel");
        self.is_flying_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/IsFlyingLabel");
        self.jumps_left_label = self.base().try_get_node_as::<Label>("MainPanel/PlayerPanel/JumpsLeftLabel");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("show_debug_ui") {
            let visible = !self.base().is_visible();
            self.base_mut().set_visible(visible);
            if visible {
                self.ensure_references();
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        if !self.base().is_visible() {
            return;
        }

        self.ensure_references();

        let mut err_text = String::new();
        if self.player_node.is_none() {
            err_text += INDENT_LABEL;
            err_text += "PLAYER NOT FOUND";
        } else if self.terrain_node.is_none() {
            err_text += INDENT_LABEL;
            err_text += "TERRAIN NODE NOT FOUND";
        }

        if let Some(mut label) = self.error_panel.clone() {
            label.set_text(&err_text);
        }

        if self.player_node.is_some() && self.terrain_node.is_some() {
            self.update_system_data();
            self.update_player_data();
        }
    }
}
