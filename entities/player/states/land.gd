extends PlayerState

func enter():
	actor.animation_player.play("land")


func update(delta: float):
	handle_gravity(delta)
	var direction := Input.get_axis("left","right")
	handle_flipping(direction)

	if not actor.animation_player.is_playing():
		actor.state_machine.transition_to("idle")
		return
	
	actor.move_and_slide()



