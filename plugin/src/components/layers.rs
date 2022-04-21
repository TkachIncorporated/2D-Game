use heron::PhysicsLayer;

#[derive(PhysicsLayer)]
pub(crate) enum Layer {
    Enemy,
    Bullet,
}
