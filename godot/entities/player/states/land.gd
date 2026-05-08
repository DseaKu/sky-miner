extends PlayerState

var land_timer: float


func enter():
	player.animation_player.play("land")
	land_timer = 0
	player.jumps_left = player.N_JUMPS_IN_ROW


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
	if Input.is_action_just_pressed("jump") and player.is_on_floor():
		player.state_machine.transition_to("jump")
		return

	land_timer += _delta
	if Input.is_action_pressed("left") or Input.is_action_pressed("right"):
		if land_timer > player.TIME_TO_LAND_RUN:
			player.state_machine.transition_to("run")
	else:
		if land_timer > player.TIME_TO_LAND_IDLE:
			player.state_machine.transition_to("idle")
	return
