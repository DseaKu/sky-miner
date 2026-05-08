extends PlayerState


func enter():
	player.animation_player.play("idle")
	player.get_node("CollisionShape2D").set_deferred("disabled", true)


func exit():
	player.get_node("CollisionShape2D").set_deferred("disabled", false)


func physics_update(_delta: float):
	if Input.is_action_pressed("up") and Input.is_action_pressed("boost"):
		player.velocity.y = -player.FLYING_SPEED * 3

	elif Input.is_action_pressed("up"):
		player.velocity.y = -player.FLYING_SPEED

	elif Input.is_action_pressed("down"):
		player.velocity.y = player.FLYING_SPEED
	else:
		player.velocity.y = 0

	if Input.is_action_pressed("left"):
		player.velocity.x = -player.FLYING_SPEED

	elif Input.is_action_pressed("right"):
		player.velocity.x = player.FLYING_SPEED
	else:
		player.velocity.x = 0

	player.move_and_slide()


func handle_transitions(_delta: float):
	if Input.is_action_just_pressed("jump"):
		player.state_machine.transition_to("idle")
		return
