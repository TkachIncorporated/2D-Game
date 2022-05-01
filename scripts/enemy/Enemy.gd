extends "res://scripts/enemy/Being.gd"

var stands = true
var destination = Vector2()
var velocity = Vector2()
var prev_pos = Vector2()

var target_intercepted = false
var can_bite = true
var target_obviously = false
var can_attack = true

func _ready() -> void:
	self.speed = 700
	self.hp = 90
	set_start_hp(self.hp,self.max_hp)
	
func _process(delta):
	base_attack()
	death_check()
	
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
	
func bite(targ):
	targ.reduce_hp(bite_strength)
	can_bite = false
	$BiteTimer.start(2)

func cancel_move():
	velocity = Vector2()
	destination = Vector2()
	
func set_destination(dest):
	destination=dest
	velocity = (destination - position).normalized()*speed
	stands = false
	

func _on_BiteTimer_timeout():
	can_bite = true

func _on_DetectionArea_area_entered(area: Area2D) -> void:
	if area.get_parent().get_parent().name == target_name:
		target = area.get_parent().get_parent()
		target_obviously = true
		set_destination(target.position)
	pass # Replace with function body.

func _on_DetectionArea_area_exited(area: Area2D) -> void:
	if area.get_parent().get_parent() == target:
		target_obviously = false
		$TimerSearch.start(4)
		velocity=-velocity
	pass # Replace with function body.


func _on_AttackArea_area_entered(area: Area2D) -> void:
	if area.get_parent().get_parent() == target:
		target_intercepted = true	
	
	if area.is_in_group("Weapon"):
		reduce_hp(area.get_parent().get_parent().DAMAGE)


func _on_AttackArea_area_exited(area: Area2D) -> void:
	if area.get_parent().get_parent() == target:
		target_intercepted = false	
	pass # Replace with function body.

func _on_HitBox_area_entered(area: Area2D) -> void:
	print("2")
	if area.is_in_group("Weapon"):
		print("wdqd")
		reduce_hp(area.get_parent().get_parent().DAMAGE)
		
func _on_AttackTimer_timeout() -> void:
	can_attack = true
	pass # Replace with function body.


func _on_TimerSearch_timeout() -> void:
	cancel_move()
	pass # Replace with function body.
