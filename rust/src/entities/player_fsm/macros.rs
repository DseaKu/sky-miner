macro_rules! play_animation {
    ($player:expr, $anim:expr) => {
        if let Some(mut anim_node) = $player.get_node_or_null("AnimationPlayer") {
            anim_node.call("play", &[Variant::from($anim)]);
        }
    };
}
pub(crate) use play_animation;
