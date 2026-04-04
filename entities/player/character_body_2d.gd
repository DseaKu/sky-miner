extends CharacterBody2D

const SPEED = 300.0
const JUMP_VELOCITY = -400.0

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

func _physics_process(_delta: float) -> void:
	# All physics and animations are now handled by the StateMachine's child states.
	pass
