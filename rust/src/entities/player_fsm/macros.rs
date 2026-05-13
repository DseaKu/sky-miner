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

pub(crate) use flip_sprite;
pub(crate) use play_animation;
