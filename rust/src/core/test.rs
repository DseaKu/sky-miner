use godot::prelude::*;

// 1. Define the enum and how Godot should read it (as an integer).
#[derive(GodotConvert, Var, Clone, Export)]
#[godot(via = i64)]
pub enum MyEnum {
    A,
    B,
    C,
}

// 2. Create the Node that will "host" the enum for Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Test {
    // This gives you a nice dropdown menu in the Godot Inspector!
    #[export]
    pub current_state: MyEnum,

    base: Base<Node>,
}

#[godot_api]
impl INode for Test {
    fn init(base: Base<Node>) -> Self {
        Self {
            current_state: MyEnum::A,
            base,
        }
    }
}

// 3. Expose the enum values to GDScript so they can be typed in code.
#[godot_api]
impl Test {
    #[constant]
    const ENUM_A: i64 = MyEnum::A as i64;

    #[constant]
    const ENUM_B: i64 = MyEnum::B as i64;

    #[constant]
    const ENUM_C: i64 = MyEnum::C as i64;
}
