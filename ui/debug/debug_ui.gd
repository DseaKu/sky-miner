extends CanvasLayer

@onready var system_header=$MainPanel/SystemPanel/SystemHeader
@onready var fps_label = $MainPanel/SystemPanel/FPSLabel
@onready var memory_label = $MainPanel/SystemPanel/MemoryLabel

@onready var player_header = $MainPanel/PlayerPanel/PlayerHeader
@onready var pos_label = $MainPanel/PlayerPanel/PosLabel
@onready var grid_pos_label = $MainPanel/PlayerPanel/GridPosLabel
@onready var state_label = $MainPanel/PlayerPanel/StateLabel
@onready var velocity_label = $MainPanel/PlayerPanel/VelocityLabel


const INDENT_LABEL =  "  "

func _ready() -> void:
	visible =false

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("show_debug_ui"):
		visible = not visible

func _process(_delta: float) -> void:

	if not visible:
		return
		
	update_system_data()
	update_player_data()


func update_system_data()-> void:
	system_header.text = "System"
	fps_label.text = INDENT_LABEL+"FPS: " + str(Engine.get_frames_per_second())

	# Get memory in bytes and convert to Megabytes
	var mem_bytes = OS.get_static_memory_usage()
	var mem_mb = mem_bytes / 1048576.0
	memory_label.text = INDENT_LABEL+"FPS: " + str(snapped(mem_mb, 0.01)) + " MB"


func update_player_data()->void:
	player_header.text = "Player"
	var player = get_tree().get_first_node_in_group("player")

	if not player:
		pos_label.text = INDENT_LABEL+"PLAYER NOT FOUND"
		return

	# Update Position
	var player_pos = round(player.global_position)
	pos_label.text = INDENT_LABEL+"Pos: (" + str(round(player_pos.x)) + ", " + str(round(player_pos.y)) + ")"

	# Update Grid Position
	var terrain_root = get_tree().get_first_node_in_group("terrain")
	if terrain_root and terrain_root.has_node("TileMapLayer"):
		var tilemap = terrain_root.get_node("TileMapLayer")
		
		# Convert global pixel position to local grid position
		var cell_pos = tilemap.local_to_map(player_pos)
		
		grid_pos_label.text = INDENT_LABEL + "Cell: (" + str(cell_pos.x) + ", " + str(cell_pos.y) + ")"
	else:
		grid_pos_label.text = INDENT_LABEL + "Cell: --"


	# Update State
	var state_machine = player.get_node("StateMachine")
	if state_machine and "current_state" in state_machine and state_machine.current_state:
		state_label.text = INDENT_LABEL+"State: " + state_machine.current_state.name
	else:
		state_label.text = INDENT_LABEL+"State: --"
	
	# Update Velocity
	var player_velo= player.velocity
	velocity_label.text = INDENT_LABEL + "Velocity: (" + str(round(player_velo.x)) + ", " + str(round(player_velo.y)) + ")"


	
