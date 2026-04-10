extends Control

@export var radius: float = 70.0
@export var input_action: String

@onready var player = get_tree().get_first_node_in_group("player")
@onready var equipment = player.get_node("Equipment")
@export var target_hand: Equipment.Hand

var selected_tool: Equipment.Tool = Equipment.Tool.NONE


func _ready() -> void:
	visible = false
	arrange_buttons()
	for button in get_children():
		if button is TextureButton:
			var tool_name = button.name.to_upper()
			var tool_value = Equipment.Tool.get(tool_name, Equipment.Tool.NONE)
			button.mouse_entered.connect(_on_button_mouse_entered.bind(tool_value))
			button.mouse_exited.connect(_on_button_mouse_exited.bind(tool_value))


func arrange_buttons() -> void:
	var buttons = get_children()
	var count = buttons.size()

	if count == 0:
		return

	var angle_step = TAU / count

	for i in range(count):
		var button = buttons[i] as Control
		if not button:
			continue

		var angle = (i * angle_step) - (PI / 2.0)
		var target_pos = Vector2(cos(angle), sin(angle)) * radius
		button.position = target_pos - (button.size / 2.0)


func _input(event: InputEvent) -> void:
	if event.is_action_pressed(input_action):
		visible = true
		global_position = get_viewport().get_mouse_position()
		selected_tool = Equipment.Tool.NONE

	elif event.is_action_released(input_action):
		if visible:
			_select_tool()
			visible = false


func _on_button_mouse_entered(tool: Equipment.Tool) -> void:
	selected_tool = tool
	# Optional: highlight the button


func _on_button_mouse_exited(tool: Equipment.Tool) -> void:
	if selected_tool == tool:
		selected_tool = Equipment.Tool.NONE


func _select_tool() -> void:
	if selected_tool == Equipment.Tool.NONE:
		return

	if target_hand == Equipment.Hand.LEFT:
		equipment.left_tool = selected_tool
	elif target_hand == Equipment.Hand.RIGHT:
		equipment.right_tool = selected_tool
