extends Control

enum Hand { LEFT, RIGHT }

@export var radius: float = 70.0
@export var input_action: String
@export var target_hand: Hand


func _ready() -> void:
	visible = false
	arrange_buttons()


func arrange_buttons() -> void:
	var buttons = get_children()
	var count = buttons.size()

	if count == 0:
		return

	var angle_step = TAU / count  # TAU = 2*pi

	for i in range(count):
		var button = buttons[i] as Control
		if not button:
			continue

		# Calculate the angle for this specific button.
		# Subtracting PI / 2 shifts the starting point to the top (12 o'clock)
		var angle = (i * angle_step) - (PI / 2.0)

		# Calculate X and Y coordinates using trig
		var target_pos = Vector2(cos(angle), sin(angle)) * radius

		# Offset the position by half the button's size so it centers perfectly on the coordinate
		button.position = target_pos - (button.size / 2.0)


func _input(event: InputEvent) -> void:
	if event.is_action_pressed(input_action):
		visible = true
		global_position = get_viewport().get_mouse_position()

	elif event.is_action_released(input_action):
		visible = false
