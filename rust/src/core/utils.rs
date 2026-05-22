use godot::prelude::Vector2i;
pub trait FloatExt {
    fn move_toward(self, target: f32, delta: f32) -> f32;
    fn lerp(self, to: f32, weight: f32) -> f32;
}

impl FloatExt for f32 {
    fn move_toward(self, target: f32, delta: f32) -> f32 {
        if (target - self).abs() <= delta {
            target
        } else {
            self + (target - self).signum() * delta
        }
    }
    fn lerp(self, to: f32, weight: f32) -> f32 {
        let t = weight.clamp(0.0, 1.0);
        self + (to - self) * t
    }
}

pub trait ToVector2i {
    fn to_vector2i(&self) -> Vector2i;
}

impl ToVector2i for (i32, i32) {
    fn to_vector2i(&self) -> Vector2i {
        Vector2i::new(self.0, self.1)
    }
}
