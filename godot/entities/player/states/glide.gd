extends PlayerState


func enter():
	player.animation_player.play("glide")


func physics_update(_delta: float):
	player.velocity.y = lerp(
		player.velocity.y, player.GLIDE_MAX_FALL_SPEED, _delta * player.GLIDE_FALL_ACCEL
	)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	player.velocity.x = lerp(
		player.velocity.x, direction * player.GLIDE_MAX_SPEED, player.GLIDE_ACCEL * _delta
	)

	player.move_and_slide()


func handle_transitions(_delta: float):
	if player.is_on_floor():
		player.state_machine.transition_to("land")
		return

	if player.jumps_left > 0 and Input.is_action_just_pressed("jump"):
		player.state_machine.transition_to("jump")
		return

	if not Input.is_action_pressed("up"):
		player.state_machine.transition_to("fall")
		return
