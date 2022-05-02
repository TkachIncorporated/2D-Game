extends Control

func _ready():
	$VBoxContainer/Start.grab_focus()

func _on_Start_pressed():
	var _scene = get_tree().change_scene("res://scenes/Main.tscn")

func _on_Exit_pressed():
	get_tree().quit()

func _on_Controls_pressed():
	pass
