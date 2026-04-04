extends PlayerState

var mining_timer = 0.0

func enter():
	actor.animation_player.play("mine")

func update(delta: float):
	handle_gravity(delta)
	
	if mining_timer > 0:
		mining_timer -= delta
		
	if not actor.is_on_floor():
		actor.state_machine.transition_to("fall")
		return

	if mining_timer <= 0:
		var mouse_pos = actor.get_global_mouse_position()
		if actor.global_position.distance_to(mouse_pos) <= actor.MINING_RANGE:
			if actor.terrain:
				if actor.terrain.mine_tile(mouse_pos):
					mining_timer = actor.MINING_COOLDOWN

	if not Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		actor.state_machine.transition_to("idle")
		return

	
