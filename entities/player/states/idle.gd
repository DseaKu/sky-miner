extends PlayerState

func enter():
	actor.animation_player.play("idle")

func physics_update(delta: float):
	handle_gravity(delta)
	actor.velocity.x = move_toward(actor.velocity.x, 0, actor.SPEED)
	actor.move_and_slide()

func handle_transitions():
	if not actor.is_on_floor() and actor.velocity.y > 50.0:
		actor.state_machine.transition_to("fall")
		return

	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	if Input.is_action_pressed("mine"):
		actor.state_machine.transition_to("mine")
		return
		
	if Input.is_action_pressed("left") or Input.is_action_pressed("right"):
		actor.state_machine.transition_to("run")
		return
