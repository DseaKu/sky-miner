extends PlayerState
@export var return_state: State


func enter() -> void:
	# 2. Tell the equipment component to execute the tool's actual logic
	var equipment = actor.get_node("Equipment")
	if Input.is_action_just_pressed("use_left_hand"):
		equipment.use_left_tool()
	elif Input.is_action_just_pressed("use_right_hand"):
		equipment.use_right_tool()

	# 3. Play the appropriate animation based on the tool
	# (Your equipment node could return the animation name!)
	var anim_name = equipment.get_current_tool_animation()
	actor.animation_player.play(anim_name)


func physics_update(_delta: float) -> void:
	# 4. When the animation finishes, go back to Idle
	if not actor.animation_player.is_playing():
		actor.state_machine.transition_to("idle")

# var mining_timer = 0.0
#
#
# func enter():
# 	actor.animation_player.play("mine")
#
#
# func physics_update(delta: float):
# 	handle_gravity(delta)
# 	var mouse_pos = actor.get_global_mouse_position()
#
# 	# Handle mining timer and execution
# 	if mining_timer > 0:
# 		mining_timer -= delta
#
# 	if mining_timer <= 0:
# 		if actor.global_position.distance_to(mouse_pos) <= actor.MINING_RANGE:
# 			if actor.terrain:
# 				if actor.terrain.mine_tile(mouse_pos):
# 					mining_timer = actor.MINING_COOLDOWN
#
# 	# Face the mouse while mining
# 	var face_direction = 1 if mouse_pos.x > actor.global_position.x else -1
# 	handle_flipping(face_direction)
#
# 	# Stop mine and slide
# 	actor.velocity.x = move_toward(actor.velocity.x, 0, actor.MAX_SPEED * delta)
#
# 	actor.move_and_slide()
#
#
# func handle_transitions(_delta: float):
# 	if not actor.is_on_floor():
# 		actor.state_machine.transition_to("fall")
# 		return
#
# 	if Input.is_action_just_pressed("jump") and actor.is_on_floor():
# 		actor.state_machine.transition_to("jump")
# 		return
#
# 	if not Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
# 		actor.state_machine.transition_to("idle")
# 		return
#
# 	if Input.is_action_pressed("left") or Input.is_action_pressed("right"):
# 		actor.state_machine.transition_to("run")
# 		return
