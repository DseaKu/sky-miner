extends PlayerState


func enter():
	actor.animation_player.play("idle")


func physics_update(_delta: float):
	handle_gravity(_delta)
	actor.velocity.x = move_toward(actor.velocity.x, 0, actor.GROUND_FRICTION * _delta)
	actor.move_and_slide()


func handle_transitions(_delta: float):
	if not actor.is_on_floor() and actor.velocity.y > 50.0:
		actor.state_machine.transition_to("fall")
		return

	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	if Input.is_action_just_pressed("left") or Input.is_action_just_pressed("right"):
		actor.state_machine.transition_to("run")
		return

	if Input.is_action_pressed("mine"):
		actor.state_machine.transition_to("mine")
		return
