extends PlayerState

var mining_timer = 0.0

func enter():
	if actor.animation_player.has_animation("mine"):
		actor.animation_player.play("mine")
	else:
		actor.animation_player.play("idle")

func update(delta: float):
	handle_gravity(delta)
	
	if not actor.is_on_floor() and actor.velocity.y > 50.0:
		actor.state_machine.transition_to("fall")
		return

	if not Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		actor.state_machine.transition_to("idle")
		return

	if Input.get_axis("left", "right") != 0:
		actor.state_machine.transition_to("run")
		return

	var mouse_pos = actor.get_global_mouse_position()

	if mining_timer > 0:
		mining_timer -= delta

	if mining_timer <= 0:
		if actor.global_position.distance_to(mouse_pos) <= actor.MINING_RANGE:
			if actor.terrain:
				if actor.terrain.mine_tile(mouse_pos):
					mining_timer = actor.MINING_COOLDOWN


	var direction:float= 0
	if mouse_pos.x>actor.global_position.x:
		direction = 1
	elif mouse_pos.x<actor.global_position.x: 
		direction = -1
	if direction != 0:
		handle_flipping(direction)

	# Stop mine and slide
	actor.velocity.x = move_toward(actor.velocity.x, 0, actor.SPEED)
	
	actor.move_and_slide()
