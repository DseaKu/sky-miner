extends PlayerState

func enter():
	actor.animation_player.play("fall")

func update(delta: float):
	handle_gravity(delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	actor.velocity.x = direction * actor.SPEED

	actor.move_and_slide()

	if actor.is_on_floor():
		actor.state_machine.transition_to("land")
		return
