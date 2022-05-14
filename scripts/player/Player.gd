extends "res://scripts/enemy/Being.gd"

onready var sprite = $Scaling/AnimatedSprite
onready var scaling = $Scaling
onready var attack_area = $Scaling/AttackArea/AttackCollider
onready var inventory = $Inventory

const MOVEMENT_SPEED = 200
const JUMP_SPEED = 400
const GRAVITY = 1000

var velocity = Vector2.ZERO
var is_attacking = false
var in_inventory = false

func _process(delta):
	#movement section
	var is_jumping = Input.is_action_just_pressed("jump") and is_on_floor()
	
	if Input.is_action_pressed("move_right") && !is_attacking:
		velocity.x = MOVEMENT_SPEED
		sprite.play("Idle")
		scaling.scale.x = 1
	elif Input.is_action_pressed("move_left") && !is_attacking:
		velocity.x = -MOVEMENT_SPEED
		sprite.play("Idle")
		scaling.scale.x = -1
	else:
		velocity.x = 0
		if !is_attacking:
			sprite.play("Idle")
	
	if Input.is_action_just_pressed("jump") and is_jumping:
		velocity.y = -JUMP_SPEED
	
	velocity.y += GRAVITY * delta;
	velocity = move_and_slide(velocity, Vector2.UP)
	
	#attack section
	if Input.is_action_pressed("attack"):
		sprite.play("Attack")
		attack_area.disabled = false
		is_attacking = true
		inventory.add_item("Slime", 1)
	
	#inventory section
	if Input.is_action_just_pressed("ui_home") and not in_inventory:
			inventory.show()
			in_inventory = true
	elif Input.is_action_just_pressed("ui_home"):
			inventory.hide()
			in_inventory = false
	

func _on_AnimatedSprite_animation_finished():
	if sprite.animation == "Attack":
		attack_area.disabled = true
		is_attacking = false
