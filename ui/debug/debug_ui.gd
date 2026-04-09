extends CanvasLayer

@onready var error_panel = $MainPanel/ErrorPanel

@onready var system_header = $MainPanel/SystemPanel/SystemHeader
@onready var fps_label = $MainPanel/SystemPanel/FPSLabel
@onready var memory_label = $MainPanel/SystemPanel/MemoryLabel
@onready var draw_calls = $MainPanel/SystemPanel/DrawCalls
@onready var process_time = $MainPanel/SystemPanel/ProcessTime
@onready var physics_time = $MainPanel/SystemPanel/PhysicsTime

@onready var player_header = $MainPanel/PlayerPanel/PlayerHeader
@onready var pos_label = $MainPanel/PlayerPanel/PosLabel
@onready var grid_pos_label = $MainPanel/PlayerPanel/GridPosLabel
@onready var chunk_pos_label = $MainPanel/PlayerPanel/ChunkPosLabel
@onready var state_label = $MainPanel/PlayerPanel/StateLabel
@onready var velocity_label = $MainPanel/PlayerPanel/VelocityLabel

@onready var game_header = $MainPanel/GamePanel/GameHeader
@onready var isle_spawn_penality_label = $MainPanel/GamePanel/IsleSpawnPenality
@onready var rarity_factor_label = $MainPanel/GamePanel/RarityFactor
@onready var ore_thresh_label = $MainPanel/GamePanel/OreTreshLabel
@onready var gem_thresh_label = $MainPanel/GamePanel/GemTreshLabel

const INDENT_LABEL = "   "

var player: CharacterBody2D
var terrain: Node2D


func _ready() -> void:
	visible = false


func _input(event: InputEvent) -> void:
	if event.is_action_pressed("show_debug_ui"):
		visible = not visible
		if visible:
			# Refresh references when UI is opened
			_ensure_references()


func _ensure_references() -> void:
	if not player:
		player = get_tree().get_first_node_in_group("player") as CharacterBody2D
	if not terrain:
		terrain = get_tree().get_first_node_in_group("terrain") as Node2D


func _process(_delta: float) -> void:
	if not visible:
		return

	var err_text = ""
	_ensure_references()
	if not player:
		err_text.text += INDENT_LABEL + "PLAYER NOT FOUND"
		return
	if not terrain:
		err_text.text += INDENT_LABEL + "TERRAIN NODE NOT FOUND"
		return
	error_panel.text = err_text
	if not (terrain or player):
		return

	update_system_data()
	update_player_data()
	update_game_data()


func update_system_data() -> void:
	system_header.text = "System:"
	fps_label.text = INDENT_LABEL + "FPS: " + str(Engine.get_frames_per_second())

	# Get memory in bytes and convert to Megabytes
	var mem_bytes = OS.get_static_memory_usage()
	var mem_mb = mem_bytes / 1048576.0
	memory_label.text = INDENT_LABEL + "Memory: " + str(snapped(mem_mb, 0.01)) + " MB"

	# GPU: Draw Calls
	var calls = Performance.get_monitor(Performance.RENDER_TOTAL_DRAW_CALLS_IN_FRAME)
	draw_calls.text = INDENT_LABEL + "Draw Calls: " + str(calls)

	# CPU: Process and Physics times (converted from seconds to milliseconds)
	var p_time = Performance.get_monitor(Performance.TIME_PROCESS) * 1000.0
	process_time.text = INDENT_LABEL + "Process: " + str(snapped(p_time, 0.01)) + " ms"

	var ph_time = Performance.get_monitor(Performance.TIME_PHYSICS_PROCESS) * 1000.0
	physics_time.text = INDENT_LABEL + "Physics: " + str(snapped(ph_time, 0.01)) + " ms"


func update_game_data() -> void:
	game_header.text = "Game:"
	isle_spawn_penality_label.text = (
		INDENT_LABEL + "Height Penalty:" + str(snapped(terrain.height_penalty, 0.001))
	)
	rarity_factor_label.text = (
		INDENT_LABEL + "Rarity Factor:" + str(snapped(terrain.rarity_factor, 0.001))
	)
	ore_thresh_label.text = (
		INDENT_LABEL + "Ore Threshold:" + str(snapped(terrain.ore_threshold, 0.001))
	)
	gem_thresh_label.text = (
		INDENT_LABEL + "Gem Threshold:" + str(snapped(terrain.gem_threshold, 0.001))
	)


func update_player_data() -> void:
	player_header.text = "Player:"

	# Update Position
	var player_pos = round(player.global_position)
	pos_label.text = (
		INDENT_LABEL + "Pos: (" + str(round(player_pos.x)) + ", " + str(round(player_pos.y)) + ")"
	)

	# Update Grid and Chunk Position
	if terrain.has_node("TileMapLayer"):
		var tilemap = terrain.get_node("TileMapLayer")

		# Convert global pixel position to local grid position
		var cell_pos = tilemap.local_to_map(player_pos)

		grid_pos_label.text = (
			INDENT_LABEL + "Cell: (" + str(cell_pos.x) + ", " + str(cell_pos.y) + ")"
		)

		# Calculate Chunk Position
		# Grabs the CHUNK_SIZE constant directly from your terrain.gd script
		var chunk_size = terrain.CHUNK_SIZE
		var chunk_x = floori(cell_pos.x / float(chunk_size))
		var chunk_y = floori(cell_pos.y / float(chunk_size))

		chunk_pos_label.text = (
			INDENT_LABEL + "Chunk: (" + str(chunk_x) + ", " + str(chunk_y) + ")"
		)
	else:
		grid_pos_label.text = INDENT_LABEL + "Cell: --"
		chunk_pos_label.text = INDENT_LABEL + "Chunk: --"

	# Update State
	var state_machine = player.get_node("StateMachine")
	if state_machine and "current_state" in state_machine and state_machine.current_state:
		state_label.text = INDENT_LABEL + "State: " + state_machine.current_state.name
	else:
		state_label.text = INDENT_LABEL + "State: --"

	# Update Velocity
	var player_velo = player.velocity
	velocity_label.text = (
		INDENT_LABEL
		+ "Velocity: ("
		+ str(round(player_velo.x))
		+ ", "
		+ str(round(player_velo.y))
		+ ")"
	)
