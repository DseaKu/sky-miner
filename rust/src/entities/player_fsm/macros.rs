macro_rules! play_animation {
    ($player:expr, $anim:expr) => {
        if let Some(mut anim_node) = $player.get_node_or_null("AnimationPlayer") {
            anim_node.call("play", &[Variant::from($anim)]);
        }
    };
}

macro_rules! flip_sprite {
    ($player:expr, $direction:expr) => {
        if let Some(mut sprite) = $player.get_node_or_null("Sprite2D") {
            if $direction > 0.0 {
                sprite.set("flip_h", &Variant::from(false));
            } else if $direction < 0.0 {
                sprite.set("flip_h", &Variant::from(true));
            }
        }
    };
}
macro_rules! apply_gravity {
    ($velocity_y:expr, $delta:expr) => {
        $velocity_y = $crate::core::utils::FloatExt::lerp(
            $velocity_y,
            $crate::entities::player_fsm::constants::in_air::MAX_SPEED,
            $crate::entities::player_fsm::constants::in_air::ACCEL * $delta as f32,
        )
    };
}

macro_rules! transition {
    ($variant:ident, $state:expr) => {
        return Some($crate::entities::player_fsm::State::$variant($state))
    };
}

pub(crate) use apply_gravity;
pub(crate) use flip_sprite;
pub(crate) use play_animation;
pub(crate) use transition;
