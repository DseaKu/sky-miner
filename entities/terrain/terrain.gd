extends Node2D

@onready var tile_map = $TileMapLayer

# Chunk Generation Settings
const CHUNK_SIZE = 16
const RENDER_DISTANCE = 10

# Atlas Coords
const TILE_SOURCE_ID = 2
const DIRT = Vector2i(0, 0)
const STONE = Vector2i(1, 0)
const ORE = Vector2i(2, 0)
const EMPTY_CELL = Vector2i(3, 3)
const NONE_EXISTING_CELL = Vector2i(-1, -1)

# Noise Thresholds
const ORE_SEED = 1
const ORE_SPREAD = 0.15
const ORE_THRESHOLD = 0.6
const DIRT_THRESHOLD = -0.3

const EMPTY_CELLS_SEED = 2
const EMPTY_CELLS_SPREAD = 0.1
const EMPTY_CELLS_THRESHOLD = 0.30

const ISLAND_SEED = 3
const ISLAND_SPREAD = 0.0025
const ISLAND_THRESHOLD = 0.5
const ISLAND_STRETCH_X = 2.0
const ISLAND_STRETCH_Y = 25.0
const SPACE_ISLE_GROUND = -7
const PENALITY_STEP_FACTOR = .000003

# Generators
var ore_noise = FastNoiseLite.new()
var void_noise = FastNoiseLite.new()
var island_noise = FastNoiseLite.new()

# Track generation state
var generated_chunks: Dictionary = {}
var player_node: Node2D = null
var current_player_chunk: Vector2i = Vector2i(0, -1)

# Isle Gen Dynamic Parameters
var island_spread = ISLAND_SPREAD
var island_stretch_y = ISLAND_STRETCH_Y
var isle_spawn_penality := 0.0


func _ready() -> void:
	add_to_group("terrain")
	setup_noise()


func setup_noise() -> void:
	ore_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	ore_noise.seed = ORE_SEED
	ore_noise.frequency = ORE_SPREAD

	void_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	void_noise.seed = EMPTY_CELLS_SEED
	void_noise.frequency = EMPTY_CELLS_SPREAD

	island_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	island_noise.seed = ISLAND_SEED
	island_noise.frequency = ISLAND_SPREAD


func _process(_delta: float) -> void:
	if not player_node:
		player_node = get_tree().get_first_node_in_group("player")
		return

	# Get player's grid position
	var local_pos = tile_map.to_local(player_node.global_position)
	var grid_pos = tile_map.local_to_map(local_pos)

	# Calculate which chunk the player is currently in
	var new_chunk_x = floori(grid_pos.x / float(CHUNK_SIZE))
	var new_chunk_y = floori(grid_pos.y / float(CHUNK_SIZE))
	var new_player_chunk = Vector2i(new_chunk_x, new_chunk_y)

	# Only update terrain if the player crossed into a new chunk
	if new_player_chunk != current_player_chunk:
		current_player_chunk = new_player_chunk
		update_chunks()


func update_chunks() -> void:
	# Loop through the render distance radius
	for cx in range(
		current_player_chunk.x - RENDER_DISTANCE, current_player_chunk.x + RENDER_DISTANCE + 1
	):
		for cy in range(
			current_player_chunk.y - RENDER_DISTANCE, current_player_chunk.y + RENDER_DISTANCE + 1
		):
			var chunk_coord = Vector2i(cx, cy)

			# Generate the chunk if it hasn't been generated yet
			if not generated_chunks.has(chunk_coord):
				generate_chunk(chunk_coord)


func generate_chunk(chunk_coord: Vector2i) -> void:
	# Mark chunk as generated
	generated_chunks[chunk_coord] = true

	# Calculate the starting global grid coordinates for this specific chunk
	var start_x = chunk_coord.x * CHUNK_SIZE
	var start_y = chunk_coord.y * CHUNK_SIZE

	isle_spawn_penality = move_toward(ISLAND_THRESHOLD, 0.0, player_node.global_position.y * .00001)

	for x in range(start_x, start_x + CHUNK_SIZE):
		for y in range(start_y, start_y + CHUNK_SIZE):
			var grid_position = Vector2i(x, y)

			if y > 0:
				tile_map.set_cell(grid_position, TILE_SOURCE_ID, STONE)
				continue

			if y > SPACE_ISLE_GROUND:
				tile_map.set_cell(grid_position, TILE_SOURCE_ID, EMPTY_CELL)
				continue

			# Apply the stretch multipliers to the coordinates
			var island_val = island_noise.get_noise_2d(x * ISLAND_STRETCH_X, y * ISLAND_STRETCH_Y)

			if island_val < isle_spawn_penality:
				continue

			var block_type = STONE

			# Apply voids and ore only to the underground stone layer
			var ore_noise_val = ore_noise.get_noise_2d(x, y)
			var void_noise_val = void_noise.get_noise_2d(x, y)

			if void_noise_val > EMPTY_CELLS_THRESHOLD:
				block_type = EMPTY_CELL
			elif ore_noise_val < DIRT_THRESHOLD:
				block_type = DIRT
			elif ore_noise_val > ORE_THRESHOLD:
				block_type = ORE

			tile_map.set_cell(grid_position, TILE_SOURCE_ID, block_type)


func mine_tile(world_position: Vector2) -> bool:
	var local_pos = tile_map.to_local(world_position)
	var grid_pos = tile_map.local_to_map(local_pos)

	# Check if there is a tile there
	var tar_cell = tile_map.get_cell_atlas_coords(grid_pos)
	if tar_cell != EMPTY_CELL and tar_cell != NONE_EXISTING_CELL:
		var atlas_coords = tile_map.get_cell_atlas_coords(grid_pos)
		if atlas_coords == ORE:
			print("Mined Ore!")
		elif atlas_coords == STONE:
			print("Mined Stone.")
		elif atlas_coords == DIRT:
			print("Mined Dirt.")

		tile_map.set_cell(grid_pos, TILE_SOURCE_ID, EMPTY_CELL)
		return true
	return false
