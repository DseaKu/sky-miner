extends PlayerState

func enter():
	actor.animation_player.play("jump")
	actor.velocity.y = actor.JUMP_VELOCITY

func update(delta: float):
	handle_gravity(delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	actor.velocity.x = direction * actor.SPEED

	actor.move_and_slide()

	if actor.velocity.y >= 0:
		actor.state_machine.transition_to("fall")
		return
