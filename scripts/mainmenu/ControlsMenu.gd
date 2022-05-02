extends "res://scripts/mainmenu/MainMenu.gd"

func _ready():
	$Back.grab_focus()

func _on_Button_pressed():
	$Controls.visible = false
	$VBoxContainer.visible = true
