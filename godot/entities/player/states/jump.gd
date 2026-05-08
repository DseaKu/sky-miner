extends PlayerState

var jump_timer: float
var jump_released: float
var jump_accel: float
var jump_max_speed: float


func enter():
	jump_timer = 0
	jump_released = false
	player.velocity.y = player.JUMP_INIT_SPEED

	if player.jumps_left == player.N_JUMPS_IN_ROW:
		player.animation_player.play("jump")
	else:
		player.animation_player.play("air_slam")


func physics_update(_delta: float):
	# Y-Movement
	jump_timer += _delta
	jump_accel = player.JUMP_ACCEL
	jump_max_speed = player.JUMP_MAX_SPEED

	if Input.is_action_just_released("jump"):
		jump_released = true

	if Input.is_action_pressed("boost"):
		jump_max_speed *= player.BOOST_MAX_SPEED_FACTOR
		jump_accel *= player.BOOST_ACCEL_FACTOR

	if jump_timer < player.JUMP_MAX_DURATION and not jump_released:
		player.velocity.y = lerp(player.velocity.y, jump_max_speed, jump_accel * _delta)

	else:
		handle_gravity(_delta)

	# X-Movement
	var direction := Input.get_axis("left", "right")
	handle_flipping(direction)
	if direction != 0:
		player.velocity.x = move_toward(
			player.velocity.x, direction * player.MAX_SPEED, player.AIR_ACCEL * _delta
		)
	else:
		player.velocity.x = move_toward(player.velocity.x, 0, player.AIR_FRICTION * _delta)

	player.move_and_slide()


func handle_transitions(_delta: float):
	if player.velocity.y >= 0:
		player.state_machine.transition_to("fall")
		return
