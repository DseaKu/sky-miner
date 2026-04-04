extends CharacterBody2D

const SPEED = 300.0
const JUMP_VELOCITY = -400.0
const TIME_TO_LAND = 0.2

@onready var animation_player = $AnimationPlayer
@onready var sprite = $Sprite2D
@onready var state_machine = $StateMachine

