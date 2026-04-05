extends PlayerState

var land_timer :float

func enter():
	actor.animation_player.play("land")
	land_timer = actor.TIME_TO_LAND

func physics_update(delta: float):
	handle_gravity(delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)

	if direction != 0:
		var accel = actor.GROUND_ACCEL
		if sign(direction) != sign(actor.velocity.x) and actor.velocity.x != 0:
			accel = actor.TURN_ACCEL
		
		actor.velocity.x = move_toward(actor.velocity.x, direction * actor.SPEED, accel * delta)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.GROUND_FRICTION * delta)

	actor.move_and_slide()

func handle_transitions(delta:float):
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		actor.state_machine.transition_to("mine")
		return

	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	if Input.is_action_pressed("left") or Input.is_action_pressed("right"):
		actor.state_machine.transition_to("run")
		return
	

