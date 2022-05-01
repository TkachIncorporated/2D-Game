extends Node2D

onready var camera = $Camera2D
onready var player = $Player

func _ready():
	passівів

func _process(_delta):
	camera.set_position(player.get_position())
