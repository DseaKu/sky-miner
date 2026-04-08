extends PlayerState


func enter():
	actor.animation_player.play("glide")


func physics_update(_delta: float):
	actor.velocity.y = lerp(
		actor.velocity.y, actor.GLIDE_MAX_FALL_SPEED, _delta * actor.GLIDE_FALL_ACCEL
	)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	actor.velocity.x = lerp(
		actor.velocity.x, direction * actor.GLIDE_MAX_SPEED, actor.GLIDE_ACCEL * _delta
	)

	actor.move_and_slide()


func handle_transitions(_delta: float):
	if actor.is_on_floor():
		actor.state_machine.transition_to("land")
		return

	if actor.jumps_left > 0 and Input.is_action_just_pressed("jump"):
		actor.state_machine.transition_to("jump")
		return

	if not Input.is_action_pressed("up"):
		actor.state_machine.transition_to("fall")
		return
