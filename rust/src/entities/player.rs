use godot::classes::ISprite2D;
use godot::classes::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    angular_speed: f32,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            angular_speed: 3.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f32) {
        let radians = self.angular_speed * delta;
        self.base_mut().rotate(radians);
    }
}
