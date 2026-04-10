class_name Equipment
extends Node

enum Tool { NONE, PICKAXE, HARPOON, BINOCULARS }
enum Hand { LEFT, RIGHT }

@export var left_tool := Tool.PICKAXE
@export var right_tool := Tool.PICKAXE
@onready var pickaxe := $Pickaxe

@onready var player = get_parent()
@onready var terrain = get_tree().get_first_node_in_group("terrain")


func _ready() -> void:
	if pickaxe:
		pickaxe.setup(player, terrain)


func _input(_event):
	if Input.is_action_just_pressed("use_left_hand"):
		use_tool(Hand.LEFT)

	if Input.is_action_just_pressed("use_right_hand"):
		use_tool(Hand.RIGHT)


func use_tool(hand: Hand) -> void:
	var tool = left_tool if hand == Hand.LEFT else right_tool

	match tool:
		Tool.NONE:
			pass
		Tool.PICKAXE:
			pickaxe.use()
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
