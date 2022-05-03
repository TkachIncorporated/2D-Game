extends "res://scripts/enemy/Being.gd"

var stands = true
var destination = Vector2()
var velocity = Vector2()
var prev_pos = Vector2()

var target_intercepted = false
var can_bite = true
var target_obviously = false
var can_attack = true

const GRAVITY = 1000

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
	

func _on_BiteTimer_timeout() -> void:
	can_bite = true
	pass

func _on_DetectionArea_area_entered(area: Area2D) -> void:
	if area.is_in_group("Player"):
		target = area.get_parent().get_parent()
		target_obviously = true
		set_destination(target.position)
	pass # Replace with function body.

func _on_DetectionArea_area_exited(area: Area2D) -> void:
	if area.is_in_group("Player"):
		target_obviously = false
		$TimerSearch.start(4)
		velocity=-velocity
	pass # Replace with function body.


func _on_AttackArea_area_entered(area: Area2D) -> void:
	if area.is_in_group("Player"):
		target_intercepted = true	
	pass # Replace with function body.


func _on_AttackArea_area_exited(area: Area2D) -> void:
	if area.is_in_group("Player"):
		target_intercepted = false	
	pass # Replace with function body.

func _on_HitBox_area_entered(area: Area2D) -> void:
	if area.is_in_group("Weapon"):
		reduce_hp(area.get_parent().get_parent().DAMAGE)
		
func _on_AttackTimer_timeout() -> void:
	can_attack = true
	pass # Replace with function body.


func _on_TimerSearch_timeout() -> void:
	target_obviously = false
	cancel_move()
	pass # Replace with function body.
