extends PlayerState

func enter():
	actor.animation_player.play("fall")

func physics_update(delta: float):
	handle_gravity(delta)

	var direction := Input.get_axis("left", "right")

	if direction != 0:
		actor.velocity.x = move_toward(actor.velocity.x, direction * actor.SPEED, actor.AIR_ACCEL * delta)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.AIR_FRICTION * delta)

	actor.move_and_slide()

func handle_transitions():
	if actor.is_on_floor():
		actor.state_machine.transition_to("land")
