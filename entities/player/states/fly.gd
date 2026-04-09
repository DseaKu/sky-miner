extends PlayerState


func enter():
	actor.animation_player.play("idle")
	actor.get_node("CollisionShape2D").set_deferred("disabled", true)


func exit():
	actor.get_node("CollisionShape2D").set_deferred("disabled", false)


func physics_update(_delta: float):
	if Input.is_action_pressed("up"):
		actor.velocity.y = -actor.FLYING_SPEED

	elif Input.is_action_pressed("down"):
		actor.velocity.y = actor.FLYING_SPEED
	else:
		actor.velocity.y = 0

	if Input.is_action_pressed("left"):
		actor.velocity.x = -actor.FLYING_SPEED

	elif Input.is_action_pressed("right"):
		actor.velocity.x = actor.FLYING_SPEED
	else:
		actor.velocity.x = 0

	actor.move_and_slide()


func handle_transitions(_delta: float):
	if Input.is_action_just_pressed("jump"):
		actor.state_machine.transition_to("idle")
		return
