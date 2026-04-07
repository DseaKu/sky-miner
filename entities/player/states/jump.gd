extends PlayerState

func enter():
	actor.animation_player.play("jump")
	actor.velocity.y = actor.JUMP_VELOCITY

func physics_update(_delta: float):
	handle_gravity(_delta)

	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	
	if direction != 0:
		actor.velocity.x = move_toward(actor.velocity.x, direction * actor.SPEED, actor.AIR_ACCEL * _delta)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.AIR_FRICTION * _delta)

	if Input.is_action_just_released("jump") and actor.velocity.y < 0:
		actor.velocity.y *= actor.JUMP_RELEASE_FORCE

	actor.move_and_slide()

func handle_transitions(_delta: float):
	if actor.velocity.y >= 0:
		actor.state_machine.transition_to("fall")
