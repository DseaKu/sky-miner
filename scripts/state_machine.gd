class_name StateMachine
extends Node

@export var initial_state: State
var current_state: State
var states: Dictionary = {}


func _ready():
	await get_parent().ready
	for child in get_children():
		if child.has_method("enter"):
			var key = child.name.to_snake_case()
			states[key] = child
			child.actor = get_parent()

	if initial_state:
		current_state = initial_state
		current_state.enter()


func _physics_process(delta):
	if current_state:
		current_state.handle_transitions(delta)
		current_state.physics_update(delta)


func transition_to(state_name: String):
	var new_state = states.get(state_name)
	if !new_state:
		return

	if current_state:
		current_state.exit()

	current_state = new_state
	current_state.enter()
