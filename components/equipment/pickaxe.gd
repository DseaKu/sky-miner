extends Node

const MINING_RANGE := 100.0

@onready var terrain = get_tree().get_first_node_in_group("terrain")
@onready var player = get_parent().get_parent()


func use() -> void:
	if not player or not terrain:
		return

	var mouse_pos = player.get_global_mouse_position()
	var dist = player.global_position.distance_to(mouse_pos)

	if dist <= MINING_RANGE:
		terrain.mine_tile(mouse_pos)
