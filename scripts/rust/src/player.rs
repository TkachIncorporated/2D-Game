use gdnative::{
    api::{CollisionShape2D, GlobalConstants},
    prelude::*,
};

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
    fn _input(&mut self, _owner: &KinematicBody2D, event: Ref<InputEvent>) {
        let e = unsafe { event.assume_safe() };

        if let Some(v) = e.cast::<InputEventKey>() {
            let key_code = v.scancode();
            let value = v.is_pressed();
            let sprite = unsafe { self.sprite.assume_safe() };

            if key_code == GlobalConstants::KEY_A {
                self.controls.direction = Direction::Left;
                self.controls.left = value;
                sprite.set_flip_h(true);
            }

            if key_code == GlobalConstants::KEY_D {
                self.controls.direction = Direction::Right;
                self.controls.right = value;
                sprite.set_flip_h(false);
            }

            if !self.is_jumping && key_code == GlobalConstants::KEY_SPACE {
                self.is_jumping = true;
                self.controls.jump = value;
            }
        };
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: &KinematicBody2D, delta: f64) {
        if self.controls.left {
            self.velocity.x = -MOVEMENT_SPEED;
        }

        if self.controls.right {
            self.velocity.x = MOVEMENT_SPEED;
        }

        if self.controls.left == self.controls.right {
            self.velocity.x = 0.0;
        }

        if self.controls.jump && _owner.is_on_floor() {
            self.velocity.y = -JUMP_SPEED;
        }

        if self.is_jumping && _owner.is_on_floor() {
            self.is_jumping = false;
        }

        if self.is_jumping && _owner.is_on_floor() {
            self.is_jumping = false;
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
