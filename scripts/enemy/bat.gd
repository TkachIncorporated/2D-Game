extends "res://scripts/enemy/Enemy.gd"


func _ready() -> void:
	self.speed = 700
	self.hp = 100
	set_start_hp(self.hp,self.max_hp)
	
func _process(delta):
	base_attack()
	death_check()
	if not target_obviously:
		$Collider123.disabled = false
		velocity.y -= GRAVITY * delta;	
		velocity = move_and_slide(velocity, Vector2.UP)
	else:
		$Collider123.disabled = true
	
func base_attack():
	if velocity:
		prev_pos = position
		move_and_slide(velocity)
	
	if target_intercepted and can_bite:
		bite(target)
		
	if target_obviously and can_attack:
		can_attack = false
		$AttackTimer.start(5)
		set_destination(target.position)
	
