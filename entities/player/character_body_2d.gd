extends CharacterBody2D

# Movement
# Ground
const MAX_SPEED: float = 400.0
const GROUND_ACCEL: float = 1.5
const TURN_ACCEL: float = 10.0
const GROUND_FRICTION: float = 3000.0
const TIME_TO_LAND_IDLE: float = 0.2
const TIME_TO_LAND_RUN: float = 0.1

# In Air
const JUMP_MAX_SPEED: float = -300.0
const JUMP_MAX_DURATION: float = 0.20
const JUMP_ACCEL: float = 10.0
const N_JUMPS_IN_ROW: int = 2
const AIR_SLAM_VELOCITY: float = 600.0
const AIR_ACCEL: float = 400.0
const AIR_FRICTION: float = 70.0
const FALL_ACCEL: float = 2.5
const MAX_FALL_SPEED: float = 700.0

const MINING_RANGE: float = 100.0
const MINING_COOLDOWN: float = 0.2

var jumps_left: int = N_JUMPS_IN_ROW

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

var terrain: Node2D


func _ready() -> void:
	terrain = get_tree().get_first_node_in_group("terrain")
