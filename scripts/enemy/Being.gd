extends KinematicBody2D

var speed = 200
var is_living = true

onready var hp = 100
export var max_hp = 100
var bite_strength = 10

func set_start_hp(cur_hp, cur_max_hp):
	$HP_bar.value = cur_hp
	$HP_bar.max_value = cur_max_hp
	
func update_hp():
	$HP_bar.value = hp

func _process(_delta):
	if self.hp <= 0:
		dead()

func dead():
	is_living = false;
	print("Something is dead.")
	if !self.is_in_group("Player"):
		self.queue_free()

func reduce_hp(val):
	self.hp -= val
	print("bite")
	update_hp()
