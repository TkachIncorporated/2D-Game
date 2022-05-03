extends "res://scripts/enemy/Enemy.gd"


func _ready() -> void:
	self.speed = 0
	self.hp = 200
	self.max_hp = 200
	set_start_hp(self.hp,self.max_hp)

func _process(delta):
	base_attack()
	death_check()
	
func base_attack():
	print()
	
