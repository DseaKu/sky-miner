class_name Equipment
extends Node

enum Tool { NONE, PICKAXE, HARPOON, BINOCULARS }
enum Hand { LEFT, RIGHT }

@export var left_tool := Tool.PICKAXE
@export var right_tool := Tool.BINOCULARS
@onready var pickaxe := $Pickaxe
@onready var binoculars := $Binoculars


func _input(_event):
	if Input.is_action_just_pressed("use_left_hand"):
		use_tool(Hand.LEFT)

	if Input.is_action_just_pressed("use_right_hand"):
		use_tool(Hand.RIGHT)


func use_tool(hand: Hand) -> void:
	var tool = left_tool if hand == Hand.LEFT else right_tool

	match tool:
		Tool.PICKAXE:
			pickaxe.use()
		Tool.HARPOON:
			pass
		Tool.BINOCULARS:
			binoculars.use()
		_:
			pass
