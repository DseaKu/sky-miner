extends CharacterBody2D

const SPEED = 300.0
const JUMP_VELOCITY = -400.0

# Create a reference to the AnimationPlayer node when the scene loads
@onready var animation_player = $AnimationPlayer

func _physics_process(delta: float) -> void:
	# Add the gravity.
	if not is_on_floor():
		velocity += get_gravity() * delta

	# Handle jump.
	if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	var direction := Input.get_axis("ui_left", "ui_right")
	if direction:
		velocity.x = direction * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)

	# --- Animation Handling ---
	if is_on_floor():
		if direction == 0:
			# Play idle when standing still
			animation_player.play("idle")
		else:
			animation_player.play("run")
			# Placeholder for when you create a running animation
	else:
		# Placeholder for when you create jump/fall animations
		pass

	move_and_slide()
