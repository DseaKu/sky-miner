extends PlayerState


func enter():
	actor.animation_player.play("idle")


func physics_update(_delta: float):
	if Input.is_action_pressed("up"):
		actor.velocity.y -= 600

	if Input.is_action_pressed("left"):
		actor.velocity.x -= 600
	if Input.is_action_pressed("right"):
		actor.velocity.x += 600
	actor.move_and_slide()


func handle_transitions(_delta: float):
	if Input.is_action_pressed("right"):
		actor.state_machine.transition_to("idle")
		return
