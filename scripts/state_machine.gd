class_name StateMachine
extends Node

@export var initial_state: State
var current_state: State
var states: Dictionary = {}

func _ready():
    await get_parent().ready
    for child in get_children():
        if child.has_method("enter"):
            states[child.name.to_lower()] = child
            child.actor = get_parent()
    
    if initial_state:
        current_state = initial_state
        current_state.enter()

func _physics_process(delta):
    if current_state:
        current_state.handle_transitions(delta)
        current_state.physics_update(delta)

func transition_to(state_name: String):
    # print("Transitioning to: ", state_name)
    var new_state = states.get(state_name.to_lower())
    if !new_state:
        return
        
    if current_state:
        current_state.exit()
        
    current_state = new_state
    current_state.enter()
