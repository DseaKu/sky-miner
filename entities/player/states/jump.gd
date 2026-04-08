extends PlayerState

var jump_timer
var jump_released


func enter():
	jump_timer = 0
	jump_released = false

	if actor.jumps_left == actor.N_JUMPS_IN_ROW:
		actor.animation_player.play("jump")
	else:
		actor.animation_player.play("air_slam")


func physics_update(_delta: float):
	# Y-Movement
	jump_timer += _delta
	if Input.is_action_just_released("jump"):
		jump_released = true

	if jump_timer < actor.JUMP_MAX_DURATION and not jump_released:
		actor.velocity.y = lerp(actor.velocity.y, actor.JUMP_MAX_SPEED, 90.0 * _delta)

	else:
		handle_gravity(_delta)

	# X-Movement
	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	if direction != 0:
		actor.velocity.x = move_toward(
			actor.velocity.x, direction * actor.MAX_SPEED, actor.AIR_ACCEL * _delta
		)
	else:
		actor.velocity.x = move_toward(actor.velocity.x, 0, actor.AIR_FRICTION * _delta)

	actor.move_and_slide()


func handle_transitions(_delta: float):
	if actor.velocity.y >= 0:
		actor.state_machine.transition_to("fall")
		return
