extends KinematicBody2D

var speed = 200
var is_living = true

onready var hp = 100
export var max_hp = 100
var bite_strength = 10

var target_name = "Player"
var target = null



func set_start_hp(hp, max_hp):
	$HP_bar.value=hp
	$HP_bar.max_value=max_hp
	
func update_hp():
	$HP_bar.value = hp

func _process(delta):
	death_check()

func death_check():
	if self.hp <= 0:
		self.queue_free()

func _ready() -> void:
	target = get_parent().get_child(0)
	print(target)
	pass # Replace with function body.

func reduce_hp(val):
	self.hp-=val
	update_hp()
	print(self.hp)


