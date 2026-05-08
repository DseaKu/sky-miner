extends PlayerState


func enter():
	player.animation_player.play("air_slam")
	player.velocity.y = player.AIR_SLAM_VELOCITY


func physics_update(_delta: float):
	handle_gravity(_delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	if direction != 0:
		player.velocity.x = move_toward(
			player.velocity.x, direction * player.MAX_SPEED, player.AIR_ACCEL * _delta
		)
	else:
		player.velocity.x = move_toward(player.velocity.x, 0, player.AIR_FRICTION * _delta)

	player.move_and_slide()


func handle_transitions(_delta: float):
	if player.is_on_floor():
		player.state_machine.transition_to("land")
		return
