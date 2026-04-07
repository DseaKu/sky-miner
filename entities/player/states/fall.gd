extends PlayerState


func enter():
	actor.animation_player.play("fall")
	actor.jumps_left -= 1


func physics_update(_delta: float):
	handle_gravity(_delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	if direction != 0:
		actor.velocity.x = move_toward(
			actor.velocity.x, direction * actor.MAX_SPEED, actor.AIR_ACCEL * _delta
		)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.AIR_FRICTION * _delta)

	actor.move_and_slide()


func handle_transitions(_delta: float):
	if actor.is_on_floor():
		actor.state_machine.transition_to("land")
		return

	if actor.jumps_left > 0 and Input.is_action_just_pressed("jump"):
		actor.state_machine.transition_to("jump")
		return
