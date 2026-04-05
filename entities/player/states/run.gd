extends PlayerState

func enter():
	actor.animation_player.play("run")

func physics_update(_delta: float):
	handle_gravity(_delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	
	if direction != 0:
		var accel = actor.GROUND_ACCEL
		if sign(direction) != sign(actor.velocity.x) and actor.velocity.x != 0:
			accel = actor.TURN_ACCEL
		
		actor.velocity.x = move_toward(actor.velocity.x, direction * actor.SPEED, accel * _delta)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.GROUND_FRICTION * _delta)

	actor.move_and_slide()

func handle_transitions(_delta: float):
	if not actor.is_on_floor() and actor.velocity.y > 50.0:
		actor.state_machine.transition_to("fall")
		return

	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	if not(Input.is_action_pressed("left") or Input.is_action_pressed("right")):
		if Input.is_action_pressed("mine"):
			actor.state_machine.transition_to("mine")
			return

		actor.state_machine.transition_to("idle")
		return
