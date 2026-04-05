extends PlayerState

func enter():
	actor.animation_player.play("jump")
	actor.velocity.y = actor.JUMP_VELOCITY

func physics_update(delta: float):
	handle_gravity(delta)

	var direction := Input.get_axis("left", "right")
	
	if direction != 0:
		actor.velocity.x = move_toward(actor.velocity.x, direction * actor.SPEED, actor.AIR_ACCEL * delta)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.AIR_FRICTION * delta)

	actor.move_and_slide()

func handle_transitions():
	if actor.velocity.y >= 0:
		actor.state_machine.transition_to("fall")
