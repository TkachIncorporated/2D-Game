extends "res://scripts/enemy/Being.gd"

var stands = true
var destination = Vector2()
var velocity = Vector2()
var prev_pos = Vector2()

var target = null


var target_intercepted = false
var can_bite = true

func bite(targ):
	targ.reduce_hp(bite_strength)
	can_bite = false
	$BiteTimer.start(1)
	
func _ready() -> void:
	self.hp = 90
	set_start_hp(self.hp,self.max_hp)
	
func _process(delta):
	if velocity:
		prev_pos = position
		move_and_slide(velocity)
	wander()
	search_for_target()
	
	if target_intercepted and can_bite:
		bite(target)

func set_destination(dest):
	destination=dest
	velocity = (destination - position).normalized()*speed
	stands = false
	
func search_for_target():
	var pl = get_parent().get_child(0)
	if position.distance_to(pl.position) < 200:
		cancel_move()
		target = pl
	else:
		if target:
			cancel_move()
		target = null
	if target:
		set_destination(target.position)

func cancel_move():
	velocity = Vector2()
	destination = Vector2()
	$StandTimer.start(2)
	
func wander():
	var pos = position
	if stands:
		randomize()
		var x = int(rand_range(pos.x - 1500,pos.x+1500))
		var y = int(rand_range(pos.y - 150, pos.y+150))
		set_destination(Vector2(x,y))
	elif velocity != Vector2():
		if pos.distance_to(destination) <=speed:
			cancel_move()
		elif pos.distance_to(prev_pos)<=2:
			cancel_move()
			
func _on_StandTimer_timeout() -> void:	
	stands = true
	pass

func _on_BiteTimer_timeout() -> void:
	can_bite = true
	pass


func _on_HitBox_area_entered(area: Area2D) -> void:
	if area.get_parent() == target:
		target_intercepted = true	
	pass # Replace with function body.


func _on_AttackArea_area_exited(area: Area2D) -> void:
	if area.get_parent() == target:
		target_intercepted = false	
	pass # Replace with function body.


func _on_Timer_timeout():
	pass # Replace with function body.
