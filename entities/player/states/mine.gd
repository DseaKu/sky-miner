extends PlayerState

var mining_timer = 0.0

func enter():
	actor.animation_player.play("mine")

func update(delta: float):
	handle_gravity(delta)

