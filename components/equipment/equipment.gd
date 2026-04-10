class_name Equipment
extends Node

enum Tool { NONE, PICKAXE, HARPOON, BINOCULARS }
enum Hand { LEFT, RIGHT }

@export var left_tool := Tool.PICKAXE
@export var right_tool := Tool.BINOCULARS


func use_tool(hand: Hand) -> void:
	var tool = left_tool if hand == Hand.LEFT else right_tool

	match tool:
		Tool.NONE:
			pass
		Tool.PICKAXE:
			pass
		Tool.HARPOON:
			pass
		Tool.BINOCULARS:
			pass
		_:
			pass


func get_tool_animation(hand: Hand) -> String:
	var tool = left_tool if hand == Hand.LEFT else right_tool
	match tool:
		Tool.PICKAXE:
			return "mine"
		Tool.HARPOON:
			return "harpoon"
		Tool.BINOCULARS:
			return "binoculars"
		_:
			return "idle"
