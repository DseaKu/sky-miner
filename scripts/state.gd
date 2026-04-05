class_name State
extends Node

var actor: CharacterBody2D

# Called by the StateMachine when entering this state
func enter():
    pass

# Called by the StateMachine when leaving this state
func exit():
    pass

# Called during _physics_process for state logic
func physics_update(_delta: float):
    pass

# Called during _physics_process to handle transitions
func handle_transitions():
    pass
