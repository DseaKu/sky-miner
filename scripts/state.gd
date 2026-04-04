class_name State
extends Node

var actor: CharacterBody2D

# Called by the StateMachine when entering this state
func enter():
    pass

# Called by the StateMachine when leaving this state
func exit():
    pass

# Called during _process or _physics_process
func update(_delta: float):
    pass
