extends Node

const MINING_RANGE := 100.0

var player: CharacterBody2D
var terrain: Node2D


func setup(p_player: CharacterBody2D, p_terrain: Node2D) -> void:
	player = p_player
	terrain = p_terrain


func use() -> void:
	if not player or not terrain:
		return

	var mouse_pos = player.get_global_mouse_position()
	var dist = player.global_position.distance_to(mouse_pos)

	if dist <= MINING_RANGE:
		terrain.mine_tile(mouse_pos)
