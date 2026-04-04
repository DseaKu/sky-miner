extends CharacterBody2D

const SPEED = 300.0
const JUMP_VELOCITY = -400.0

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

func _physics_process(delta: float) -> void:
	# Add the gravity.
	if not is_on_floor():
		velocity += get_gravity() * delta

	# Handle jump.
	if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	var direction := Input.get_axis("left", "right")
	if direction:
		velocity.x = direction * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)

	# --- Mirror Sprite ---
	if direction >0:
		sprite.flip_h = false
	elif direction < 0:
		sprite.flip_h = true
		
	# --- Animation Handling ---
	if is_on_floor():
		if direction == 0:
			animation_player.play("idle")
		else:
			animation_player.play("run")
	else:
		if velocity.y < 0:
			animation_player.play("jump")
		else:
			animation_player.play("fall")

	move_and_slide()
