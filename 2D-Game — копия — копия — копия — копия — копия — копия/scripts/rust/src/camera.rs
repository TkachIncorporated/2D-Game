use gdnative::{api::Camera2D, prelude::*};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_camera)]
pub struct Camera {
    camera: Ref<Camera2D>,
    player: Ref<KinematicBody2D>,
}

#[methods]
impl Camera {
    fn register_camera(builder: &ClassBuilder<Self>) {
        builder.signal("Camera hit").done()
    }

    fn new(_owner: &Node2D) -> Self {
        Camera {
            camera: Camera2D::new().into_shared(),
            player: KinematicBody2D::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        _owner.set_physics_process(true);
        self.camera = unsafe {
            _owner
                .get_node_as::<Camera2D>("Camera2D")
                .expect("There's no Camera")
                .assume_shared()
        };

        self.player = unsafe {
            _owner
                .get_node_as::<KinematicBody2D>("Player")
                .expect("There's no Camera")
                .assume_shared()
        };
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f32) {
        let camera = unsafe { self.camera.assume_safe() };
        let player = unsafe { self.player.assume_safe() };

        camera.set_position(player.position());
    }
}
