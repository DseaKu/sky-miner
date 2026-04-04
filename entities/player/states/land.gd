extends PlayerState

var land_timer := 0.1

func enter():
	actor.animation_player.play("land")
	actor.velocity = Vector2.ZERO
	land_timer = 0.1

func update(delta: float):
	handle_gravity(delta)
	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	actor.velocity.x = direction * actor.SPEED

	actor.move_and_slide()

	land_timer -= delta
	if land_timer <= 0:
		actor.state_machine.transition_to("idle")
		return
