extends PlayerState

func enter():
	actor.animation_player.play("idle")

func update(delta: float):
	handle_gravity(delta)
	
	if Input.is_action_just_pressed("ui_accept") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	var direction := Input.get_axis("left", "right")
	if direction != 0:
		actor.state_machine.transition_to("run")
		return

	actor.velocity.x = move_toward(actor.velocity.x, 0, actor.SPEED)
	
	
	if not actor.is_on_floor() and actor.velocity.y > 50.0:
		actor.state_machine.transition_to("fall")
		return

	actor.move_and_slide()
