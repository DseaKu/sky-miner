class_name PlayerState
extends State

func handle_gravity(delta: float):
	if not actor.is_on_floor():
		actor.velocity += actor.get_gravity() * delta

func handle_flipping(direction: float):
	if direction > 0:
		actor.sprite.flip_h = false
	elif direction < 0:
		actor.sprite.flip_h = true
