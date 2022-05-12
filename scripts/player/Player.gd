extends "res://scripts/enemy/Being.gd"

onready var sprite = $Scaling/AnimatedSprite

const MOVEMENT_SPEED = 200
const JUMP_SPEED = 400
const GRAVITY = 1000

var velocity = Vector2.ZERO
var is_attacking = false;
var inventory =false ;

func _ready():
	pass 
	
func _process(delta):
	if Input.is_action_pressed("move_right") && !is_attacking:
		velocity.x = MOVEMENT_SPEED
		sprite.play("Idle")
		if $Scaling.scale.x != 1:
			$Scaling.scale.x *= -1
	elif Input.is_action_pressed("move_left") && !is_attacking:
		velocity.x = -MOVEMENT_SPEED
		sprite.play("Idle")
		if $Scaling.scale.x != -1:
			$Scaling.scale.x *= -1
	else:
		velocity.x = 0
		if !is_attacking:
			sprite.play("Idle")

		var is_jumping = Input.is_action_just_pressed("jump") and is_on_floor()
		
		if Input.is_action_just_pressed("jump") && is_jumping:
			velocity.y = -JUMP_SPEED
		
		velocity.y += GRAVITY * delta;	
		velocity = move_and_slide(velocity, Vector2.UP)
		
		if Input.is_action_pressed("attack"):
			sprite.play("Attack")
			$Scaling/AttackArea/AttackCollider.disabled = false
			is_attacking = true
			$Inventory.add_item("Slime", 1)

		if Input.is_action_just_pressed("ui_home") and not inventory:
			$Inventory.show()
			inventory=true
		elif Input.is_action_just_pressed("ui_home"):
			$Inventory.hide()
			inventory=false

	

func _on_AnimatedSprite_animation_finished():
	if sprite.animation == "Attack":
		$Scaling/AttackArea/AttackCollider.disabled = true
		is_attacking = false
