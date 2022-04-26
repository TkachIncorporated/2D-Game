use gdnative::{api::CollisionShape2D, api::KinematicBody2D, prelude::*};

use crate::controls::{Direction, KeyboardControls};

const GRAVITY: f32 = 1000.0;
const MOVEMENT_SPEED: f32 = 200.0;
const JUMP_SPEED: f32 = 400.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    is_jumping: bool,
    controls: KeyboardControls,
    velocity: Vector2,
    sprite: Ref<Sprite>,
}

#[methods]
impl Player {
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done()
    }

    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            is_jumping: false,
            controls: KeyboardControls::new(),
            velocity: Vector2::new(0.0, 0.0),
            sprite: Sprite::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        _owner.set_physics_process(true);
        self.sprite = unsafe {
            _owner
                .get_node_as::<Sprite>("Sprite")
                .expect("There's no Sprite")
                .assume_shared()
        };
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: &KinematicBody2D, delta: f64) {
        let input = Input::godot_singleton();
        let sprite = self.sprite.assume_safe();

        if input.is_action_pressed("move_left", false) {
            self.controls.direction = Direction::Left;
            self.velocity.x = -MOVEMENT_SPEED;
            sprite.set_flip_h(true);
        } else if input.is_action_pressed("move_right", false) {
            self.controls.direction = Direction::Right;
            self.velocity.x = MOVEMENT_SPEED;
            sprite.set_flip_h(false);
        } else {
            self.controls.direction = Direction::None;
            self.velocity.x = 0.0;
        }

        if input.is_action_pressed("jump", false) && _owner.is_on_floor() {
            self.is_jumping = true;
            self.velocity.y = -JUMP_SPEED;
        }

        if input.is_action_pressed("attack", false) {
            // TODO
        }

        self.velocity.y += GRAVITY * delta as f32;
        self.velocity = _owner.move_and_slide(
            self.velocity,
            Vector2::new(0.0, -1.0),
            true,
            4,
            std::f32::consts::FRAC_PI_4.into(),
            true,
        )
    }

    #[export]
    pub fn start(&self, owner: &KinematicBody2D, pos: Vector2) {
        owner.set_global_position(pos);
        owner.show();

        let collision_shape = unsafe {
            owner
                .get_node_as::<CollisionShape2D>("collision_shape_2d")
                .unwrap()
        };

        collision_shape.set_disabled(false);
    }
}
