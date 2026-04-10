extends PlayerState


func enter():
	player.animation_player.play("idle")


func physics_update(_delta: float):
	handle_gravity(_delta)
	player.velocity.x = move_toward(player.velocity.x, 0, player.GROUND_FRICTION * _delta)
	player.move_and_slide()

	if Input.is_action_just_pressed("five"):
		player.is_flying = !player.is_flying


func handle_transitions(_delta: float):
	if not player.is_on_floor() and player.velocity.y > 50.0:
		player.state_machine.transition_to("fall")
		return

	if Input.is_action_just_pressed("jump") and player.is_on_floor():
		player.state_machine.transition_to("jump")
		return

	if Input.is_action_just_pressed("left") or Input.is_action_just_pressed("right"):
		player.state_machine.transition_to("run")
		return

	if Input.is_action_pressed("use_left_tool") or Input.is_action_pressed("use_right_tool"):
		player.state_machine.transition_to("use_tool")
		return

	if player.is_flying and Input.is_action_pressed("up"):
		player.state_machine.transition_to("fly")
		return
