extends PlayerState

var land_timer :float

func enter():
	actor.animation_player.play("land")
	actor.velocity = Vector2.ZERO
	land_timer = actor.TIME_TO_LAND

func update(delta: float):
	handle_gravity(delta)
	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	actor.velocity.x = direction * actor.SPEED


	land_timer -= delta
	if land_timer <= 0:
		actor.state_machine.transition_to("idle")
		return

	if actor.velocity.x != 0:
		actor.state_machine.transition_to("run")
		return

	if Input.is_action_just_pressed("ui_accept") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	actor.move_and_slide()
