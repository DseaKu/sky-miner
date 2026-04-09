extends CharacterBody2D

# Movement
# Ground
const MAX_SPEED := 400.0
const GROUND_ACCEL := 1.5
const TURN_ACCEL := 10.0
const GROUND_FRICTION := 3000.0
const TIME_TO_LAND_IDLE := 0.2
const TIME_TO_LAND_RUN := 0.1

# In Air
const AIR_FRICTION := 40.0
const FALL_ACCEL := 3.3
const FALL_MAX_SPEED := 700.0
const AIR_SLAM_VELOCITY := 600.0
const AIR_ACCEL := 400.0

# Jump
const JUMP_MAX_SPEED := -400.0
const JUMP_ACCEL := 10.0
const JUMP_INIT_SPEED := -300.0
const JUMP_MAX_DURATION := 0.20
const BOOST_MAX_SPEED_FACTOR := 2.3
const BOOST_ACCEL_FACTOR := 1.3
const N_JUMPS_IN_ROW := 2

# Glide
const GLIDE_MAX_FALL_SPEED := 250.0
const GLIDE_FALL_ACCEL := 1.0
const GLIDE_MAX_SPEED := 300.0
const GLIDE_ACCEL := 2.0

# Fly
const IS_FLYING := false
const FLYING_SPEED := 2000.0

const MINING_RANGE := 100.0
const MINING_COOLDOWN := 0.2

var jumps_left := N_JUMPS_IN_ROW
var is_flying := IS_FLYING

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

var terrain: Node2D


func _ready() -> void:
	terrain = get_tree().get_first_node_in_group("terrain")
	if terrain:
		terrain.player_node = self
