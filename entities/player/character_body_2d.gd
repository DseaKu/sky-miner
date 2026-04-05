extends CharacterBody2D

# Movement
const SPEED = 300.0
const JUMP_VELOCITY = -400.0
const TIME_TO_LAND = 0.2
const GROUND_ACCEL = 1000.0
const GROUND_FRICTION = 3000.0
const AIR_ACCEL = 900.0
const AIR_FRICTION = 50.0


const MINING_RANGE = 100.0
const MINING_COOLDOWN = 0.2

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine


var terrain: Node2D

func _ready() -> void:
	terrain = get_tree().get_first_node_in_group("terrain")

