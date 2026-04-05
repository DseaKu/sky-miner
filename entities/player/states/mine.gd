extends PlayerState

var mining_timer = 0.0

func enter():
	if actor.animation_player.has_animation("mine"):
		actor.animation_player.play("mine")
	else:
		actor.animation_player.play("idle")

func physics_update(_delta: float):
	handle_gravity(_delta)
	var mouse_pos = actor.get_global_mouse_position()

	# Handle mining timer and execution
	if mining_timer > 0:
		mining_timer -= _delta

	if mining_timer <= 0:
		if actor.global_position.distance_to(mouse_pos) <= actor.MINING_RANGE:
			if actor.terrain:
				if actor.terrain.mine_tile(mouse_pos):
					mining_timer = actor.MINING_COOLDOWN

	# Face the mouse while mining
	var face_direction = 1 if mouse_pos.x > actor.global_position.x else -1
	handle_flipping(face_direction)

	# Stop mine and slide
	actor.velocity.x = move_toward(actor.velocity.x, 0, actor.SPEED)

	actor.move_and_slide()


func handle_transitions(_delta: float):

	if not actor.is_on_floor():
		actor.state_machine.transition_to("fall")
		return

	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
		actor.state_machine.transition_to("jump")
		return

	if not Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		actor.state_machine.transition_to("idle")
		return

	if Input.is_action_pressed("left") or Input.is_action_pressed("right"):
		actor.state_machine.transition_to("run")
		return
