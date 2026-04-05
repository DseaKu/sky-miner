extends CanvasLayer

# Create references to your labels
@onready var fps_label = $FPSLabel
@onready var pos_label = $PosLabel

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	visible =false


# func _input(event: InputEvent) -> void:
# 	if event.is_action_just_pressed("show_debug_ui"):
# 		visible = not visible

func _process(_delta: float) -> void:
	# Do not process updates if the UI is hidden
	if not visible:
		return
		
	# Update FPS
	fps_label.text = "FPS: " + str(Engine.get_frames_per_second())
	
	# Update Player Position
	var player = get_tree().get_first_node_in_group("player")
	
	if player:
		var pos_x = round(player.global_position.x)
		var pos_y = round(player.global_position.y)
		pos_label.text = "Pos: (" + str(pos_x) + ", " + str(pos_y) + ")"
	else:
		pos_label.text = "Pos: Player not found"
