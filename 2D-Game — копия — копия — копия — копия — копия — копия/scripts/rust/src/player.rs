use gdnative::{
    api::{AnimatedSprite, CollisionObject, CollisionShape2D},
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
    is_attacking: bool,
    controls: KeyboardControls,
    velocity: Vector2,
    sprite: Ref<AnimatedSprite>,
    attack_collision: Ref<CollisionShape2D>,
    scaling: Ref<Node2D>,
}

#[methods]
impl Player {
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done()
    }

    fn new(_owner: TRef<KinematicBody2D>) -> Self {
        Player {
            is_jumping: false,
            is_attacking: false,
            controls: KeyboardControls::new(),
            velocity: Vector2::new(0.0, 0.0),
            sprite: AnimatedSprite::new().into_shared(),
            attack_collision: CollisionShape2D::new().into_shared(),
            scaling: Node2D::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: TRef<KinematicBody2D>) {
        _owner.set_physics_process(true);

        self.sprite = unsafe {
            _owner
                .get_node_as::<AnimatedSprite>("Scaling/AnimatedSprite")
                .expect("There's no Sprite")
                .assume_shared()
        };

        self.attack_collision = unsafe {
            _owner
                .get_node_as::<CollisionShape2D>("Scaling/AttackArea/AttackCollider")
                .expect("There's no Shape")
                .assume_shared()
        };

        self.scaling = unsafe {
            _owner
                .get_node_as::<Node2D>("Scaling")
                .expect("There's no Shape")
                .assume_shared()
        };
    }

    #[export]
    unsafe fn finished(&mut self, _owned: TRef<KinematicBody2D>) {
        let sprite = self.sprite.assume_safe().as_ref();
        let shape = self.attack_collision.assume_safe();

        if sprite.animation() == GodotString::from("Attack") {
            self.is_attacking = false;

            shape.set_disabled(true);
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: TRef<KinematicBody2D>, delta: f64) {
        let input = Input::godot_singleton();
        let sprite = self.sprite.assume_safe();
        let shape = self.attack_collision.assume_safe();
        let scale = self.scaling.assume_safe();

        if input.is_action_pressed("move_left", false) && !self.is_attacking {
            self.controls.direction = Direction::Left;
            self.velocity.x = -MOVEMENT_SPEED;
            sprite.play("Idle", false);
            if scale.scale() != Vector2::new(-1., 1.) {
                scale.set_global_scale(Vector2 { x: -1., y: 1. });
            }
        } else if input.is_action_pressed("move_right", false) && !self.is_attacking {
            self.controls.direction = Direction::Right;
            self.velocity.x = MOVEMENT_SPEED;
            sprite.play("Idle", false);
            if scale.scale() != Vector2::new(1., 1.) {
                scale.set_global_scale(Vector2 { x: 1., y: 1. });
            }
        } else {
            self.controls.direction = Direction::None;
            self.velocity.x = 0.0;
            if !self.is_attacking {
                sprite.play("Idle", false);
            }
        }

        if input.is_action_just_pressed("jump", false) && _owner.is_on_floor() && !self.is_attacking
        {
            self.is_jumping = true;
            self.velocity.y = -JUMP_SPEED;
            sprite.play("Idle", false);
        }

        if input.is_action_just_pressed("attack", false) {
            sprite.play("Attack", false);
            self.is_attacking = true;
            shape.set_disabled(false);
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
    pub fn start(&self, owner: TRef<KinematicBody2D>, pos: Vector2) {
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
