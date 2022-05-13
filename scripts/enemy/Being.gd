extends KinematicBody2D

var speed = 200

onready var hp = 50
export var max_hp = 100
var bite_strength = 10

func set_start_hp(hp, max_hp):
	$HP_bar.value=hp
	$HP_bar.max_value=max_hp
	
func update_hp():
	$HP_bar.value=hp

func _ready() -> void:
	pass # Replace with function body.

func reduce_hp(val):
	self.hp-=val
	print("bite")
	update_hp()
	print(self.hp)
