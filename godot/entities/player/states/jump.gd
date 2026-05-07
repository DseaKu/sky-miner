extends PlayerState

var jump_timer: float
var jump_released: float
var jump_accel: float
var jump_max_speed: float


func enter():
	jump_timer = 0
	jump_released = false
	actor.velocity.y = actor.JUMP_INIT_SPEED

	if actor.jumps_left == actor.N_JUMPS_IN_ROW:
		actor.animation_player.play("jump")
	else:
		actor.animation_player.play("air_slam")


func physics_update(_delta: float):
	# Y-Movement
	jump_timer += _delta
	jump_accel = actor.JUMP_ACCEL
	jump_max_speed = actor.JUMP_MAX_SPEED

	if Input.is_action_just_released("jump"):
		jump_released = true

	if Input.is_action_pressed("boost"):
		jump_max_speed *= actor.BOOST_MAX_SPEED_FACTOR
		jump_accel *= actor.BOOST_ACCEL_FACTOR

	if jump_timer < actor.JUMP_MAX_DURATION and not jump_released:
		actor.velocity.y = lerp(actor.velocity.y, jump_max_speed, jump_accel * _delta)

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
