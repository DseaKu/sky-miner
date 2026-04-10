extends Node

const ZOOM_OUT_FACTOR := 0.6
const LERP_SPEED := 5.0
const OFFSET_FACTOR := 0.5

@onready var player = get_tree().get_first_node_in_group("player")
@onready var equipment = get_parent()

var camera: Camera2D
var was_using := false


func _ready() -> void:
	if player:
		camera = player.get_node("Camera2D")


func use():
	# The actual logic is handled in _process to support "hold to use"
	# and smooth transitions.
	pass


func _process(delta: float) -> void:
	if not camera or not equipment:
		return

	var is_using = false
	if equipment.left_tool == Equipment.Tool.BINOCULARS and Input.is_action_pressed("use_left_hand"):
		is_using = true
	elif equipment.right_tool == Equipment.Tool.BINOCULARS and Input.is_action_pressed("use_right_hand"):
		is_using = true

	if is_using:
		was_using = true
		var target_zoom = camera.DEFAULT_ZOOM * ZOOM_OUT_FACTOR
		camera.zoom = camera.zoom.lerp(target_zoom, delta * LERP_SPEED)

		var mouse_pos = camera.get_global_mouse_position()
		var target_offset = (mouse_pos - player.global_position) * OFFSET_FACTOR
		camera.offset = camera.offset.lerp(target_offset, delta * LERP_SPEED)
	elif was_using:
		camera.zoom = camera.zoom.lerp(camera.DEFAULT_ZOOM, delta * LERP_SPEED)
		camera.offset = camera.offset.lerp(Vector2.ZERO, delta * LERP_SPEED)

		if camera.zoom.is_equal_approx(camera.DEFAULT_ZOOM) and camera.offset.is_equal_approx(Vector2.ZERO):
			was_using = false
