extends CharacterBody2D

const SPEED = 300.0
const JUMP_VELOCITY = -400.0
const TIME_TO_LAND = 0.2

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

const MINING_RANGE = 100.0
const MINING_COOLDOWN = 0.2

var terrain: Node2D
var mining_timer = 0.0

func _ready() -> void:
	terrain = get_tree().get_first_node_in_group("terrain")

func _physics_process(delta: float) -> void:
	if mining_timer > 0:
		mining_timer -= delta
		
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) and mining_timer <= 0:
		var mouse_pos = get_global_mouse_position()
		if global_position.distance_to(mouse_pos) <= MINING_RANGE:
			if not terrain:
				terrain = get_tree().get_first_node_in_group("terrain")
			
			if terrain:
				if terrain.mine_tile(mouse_pos):
					mining_timer = MINING_COOLDOWN

