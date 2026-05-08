extends PlayerState


func enter():
	player.animation_player.play("run")


func physics_update(_delta: float):
	handle_gravity(_delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	if direction != 0:
		var accel = player.GROUND_ACCEL
		if sign(direction) != sign(player.velocity.x) and player.velocity.x != 0:
			accel = player.TURN_ACCEL

		player.velocity.x = lerp(player.velocity.x, direction * player.MAX_SPEED, accel * _delta)
	else:
		player.velocity.x = move_toward(player.velocity.x, 0.0, player.GROUND_FRICTION * _delta)

	player.move_and_slide()


func handle_transitions(_delta: float):
	if not player.is_on_floor() and player.velocity.y > 50.0:
		player.state_machine.transition_to("fall")
		return

	if Input.is_action_just_pressed("jump") and player.is_on_floor():
		player.state_machine.transition_to("jump")
		return

	if not (Input.is_action_pressed("left") or Input.is_action_pressed("right")):
		player.state_machine.transition_to("idle")
		return
