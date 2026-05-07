extends Node

const ZOOM_OUT_FACTOR := 0.5
const LERP_SPEED := 5.0
const OFFSET_FACTOR := 0.5

@onready var player = get_tree().get_first_node_in_group("player")
@onready var equipment = get_parent()
@onready var camera = player.get_node("Camera2D")

@onready var target_zoom := Vector2(
	camera.DEFAULT_ZOOM.x - ZOOM_OUT_FACTOR, camera.DEFAULT_ZOOM.y - ZOOM_OUT_FACTOR
)

var is_using := false


func _ready() -> void:
	set_process(false)


func use():
	set_process(true)
	is_using = true


func stop_use():
	is_using = false


func _process(delta: float) -> void:
	if is_using:
		camera.zoom = camera.zoom.lerp(target_zoom, delta * LERP_SPEED)

		var mouse_pos = camera.get_global_mouse_position()
		var target_offset = (mouse_pos - player.global_position) * OFFSET_FACTOR
		camera.offset = camera.offset.lerp(target_offset, delta * LERP_SPEED)

	else:
		camera.zoom = camera.zoom.lerp(camera.DEFAULT_ZOOM, delta * LERP_SPEED)
		camera.offset = camera.offset.lerp(Vector2.ZERO, delta * LERP_SPEED)

		if (
			camera.zoom.is_equal_approx(camera.DEFAULT_ZOOM)
			and camera.offset.is_equal_approx(Vector2.ZERO)
		):
			camera.zoom = camera.DEFAULT_ZOOM
			camera.offset = Vector2.ZERO
			set_process(false)
