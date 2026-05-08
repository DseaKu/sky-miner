use super::battle_fsm::{self, Behavior};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleManager {
    fsm: battle_fsm::State,
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleManager {
    fn init(base: Base<Node>) -> Self {
        let mut fsm = battle_fsm::State::Initial(battle_fsm::initial::State);
        fsm.on_enter();

        Self {
            fsm,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("BattleManager is in the Scene Tree.");
        self.send_event("StartBattle".to_string());
    }
}

#[godot_api]
impl BattleManager {
    #[signal]
    fn state_changed(new_state: String);

    #[func]
    pub fn end_turn(&mut self) {
        self.send_event("EndTurnPressed".to_string());
    }

    #[func]
    pub fn send_event(&mut self, event: String) {
        if self.fsm.process_event(&event) {
            self.on_state_changed();
        } else {
            godot_print!(
                "Event '{}' was ignored by state '{}'.",
                event,
                self.fsm.name()
            );
        }
    }

    fn on_state_changed(&mut self) {
        let state_name = self.fsm.name();
        godot_print!("Transitioned to state: {}", state_name);

        // If we just entered EnemyTurn, schedule an event 3 seconds from now
        if state_name == "EnemyTurn" {
            godot_print!("Scheduling enemy turn completion in 3 seconds...");
            let mut tree = self.base_mut().get_tree();
            let mut timer = tree.create_timer(3.0);
            
            // Connect the timer's timeout to our send_event function
            timer.connect(
                "timeout",
                &self.base().callable("on_enemy_timer_timeout")
            );
        }

        self.base_mut()
            .emit_signal("state_changed", &[state_name.to_variant()]);
    }

    #[func]
    fn on_enemy_timer_timeout(&mut self) {
        self.send_event("EnemyFinished".to_string());
    }
}
