extends PlayerState

var land_timer: float


func enter():
	actor.animation_player.play("land")
	land_timer = 0
	actor.jumps_left = actor.N_JUMPS_IN_ROW


func physics_update(_delta: float):
	handle_gravity(_delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	if direction != 0:
		var accel = actor.GROUND_ACCEL
		if sign(direction) != sign(actor.velocity.x) and actor.velocity.x != 0:
			accel = actor.TURN_ACCEL
		actor.velocity.x = lerp(actor.velocity.x, direction * actor.MAX_SPEED, accel * _delta)

	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0.0, actor.GROUND_FRICTION * _delta)

	actor.move_and_slide()


func handle_transitions(_delta: float):
	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	land_timer += _delta
	if Input.is_action_pressed("left") or Input.is_action_pressed("right"):
		if land_timer > actor.TIME_TO_LAND_RUN:
			actor.state_machine.transition_to("run")
	else:
		if land_timer > actor.TIME_TO_LAND_IDLE:
			actor.state_machine.transition_to("idle")
	return
