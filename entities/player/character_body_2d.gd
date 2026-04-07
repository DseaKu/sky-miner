extends CharacterBody2D

# Movement
# Ground
const MAX_SPEED = 300.0
const GROUND_ACCEL = 500.0
const GROUND_FRICTION = 3000.0
const TURN_ACCEL = 3000.0
const TIME_TO_LAND_IDLE = 0.2
const TIME_TO_LAND_RUN = 0.1

# In Air
const JUMP_VELOCITY = -400.0
const JUMP_RELEASE_FORCE = 0.5
const N_JUMPS_IN_ROW = 2
const AIR_SLAM_VELOCITY = 600.0
const AIR_ACCEL = 400.0
const AIR_FRICTION = 70.0
const FALL_ACCEL = 1000.0
const MAX_FALL_SPEED = 1000.0

const MINING_RANGE = 100.0
const MINING_COOLDOWN = 0.2

var jumps_left: int = N_JUMPS_IN_ROW

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

var terrain: Node2D


func _ready() -> void:
	terrain = get_tree().get_first_node_in_group("terrain")
