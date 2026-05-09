class_name PlayerState
extends State

var player:
	get:
		return actor


func handle_gravity(delta: float):
	player.velocity.y = lerp(player.velocity.y, player.FALL_MAX_SPEED, delta * player.FALL_ACCEL)


func handle_flipping(direction: float):
	if direction > 0:
		player.sprite.flip_h = false
	elif direction < 0:
		player.sprite.flip_h = true
