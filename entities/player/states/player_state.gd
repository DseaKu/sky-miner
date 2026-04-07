class_name PlayerState
extends State


func handle_gravity(delta: float):
	actor.velocity.y = move_toward(actor.velocity.y, actor.MAX_FALL_SPEED, delta * actor.FALL_ACCEL)


func handle_flipping(direction: float):
	if direction > 0:
		actor.sprite.flip_h = false
	elif direction < 0:
		actor.sprite.flip_h = true
