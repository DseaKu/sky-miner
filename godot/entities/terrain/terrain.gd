extends Node2D

@onready var tile_map = $TileMapLayer
@onready var terrain_generator = $TerrainGenerator

# Chunk Generation Settings
const CHUNK_SIZE = 16
const RENDER_DISTANCE = 10

# Atlas Coords
const TILE_SOURCE_ID = 2
const DIRT = Vector2i(0, 0)
const STONE = Vector2i(1, 0)
const ORE = Vector2i(2, 0)
const GEM = Vector2i(0, 1)
const EMPTY_CELL = Vector2i(3, 3)
const NONE_EXISTING_CELL = Vector2i(-1, -1)

# Ore
# const ORE_SEED = 1
const ORE_SPREAD = 0.05
const ORE_INIT_THRESHOLD = 1.00
const ORE_MIN_THRESHOLD = 0.60
const ORE_CURVE_STEEP_THRESH = 1.35
const DIRT_THRESHOLD = -0.3

# Void
# const EMPTY_CELLS_SEED = 2
const EMPTY_CELLS_SPREAD = 0.25
const EMPTY_CELLS_THRESHOLD = 0.23

# Gems
# const GEM_SEED = 3
const GEM_SPREAD = 0.35
const GEM_INIT_THRESHOLD = 1.15
const GEM_MIN_THRESHOLD = 0.50
const GEM_CURVE_STEEP_THRESH = 1.35

# Isle
# const ISLAND_SEED = 4
const ISLAND_SPREAD = 0.0013
const ISLAND_THRESHOLD = 0.25
const ISLAND_STRETCH_X = 4.0
const ISLAND_STRETCH_Y = 40.0
const SPACE_ISLE_GROUND = -4
const HEIGHT_PENALTY = .00001
const RARITY_HEIGHT_IMPACT := 3.0

# Generators
var ore_noise = FastNoiseLite.new()
var void_noise = FastNoiseLite.new()
var gem_noise = FastNoiseLite.new()
var island_noise = FastNoiseLite.new()

# Track generation state
var generated_chunks: Dictionary = {}
var player_node: Node2D = null
var current_player_chunk: Vector2i = Vector2i(0, -1)

# Isle Gen Dynamic Parameters
var island_spread = ISLAND_SPREAD
var island_stretch_y = ISLAND_STRETCH_Y
var height_penalty := 0.0
var rarity_factor := 0.0
var ore_threshold := 0.0
var gem_threshold := 0.0


func _ready() -> void:
	add_to_group("terrain")
	setup_noise()


func setup_noise() -> void:
	ore_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	ore_noise.seed = randi()
	ore_noise.frequency = ORE_SPREAD

	gem_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	gem_noise.seed = randi()
	gem_noise.frequency = GEM_SPREAD

	void_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	void_noise.seed = randi()
	void_noise.frequency = EMPTY_CELLS_SPREAD

	island_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	island_noise.seed = randi()
	island_noise.frequency = ISLAND_SPREAD


func _process(_delta: float) -> void:
	return
	if not player_node:
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
		cleanup_chunks()


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


func cleanup_chunks() -> void:
	# Define a buffer outside the render distance for unloading
	var unload_distance = RENDER_DISTANCE + 2
	var chunks_to_remove: Array[Vector2i] = []

	for chunk_coord in generated_chunks:
		var distance = (chunk_coord - current_player_chunk).abs()
		if distance.x > unload_distance or distance.y > unload_distance:
			chunks_to_remove.append(chunk_coord)

	for chunk_coord in chunks_to_remove:
		unload_chunk(chunk_coord)
		generated_chunks.erase(chunk_coord)


func unload_chunk(chunk_coord: Vector2i) -> void:
	var start_x = chunk_coord.x * CHUNK_SIZE
	var start_y = chunk_coord.y * CHUNK_SIZE

	for x in range(start_x, start_x + CHUNK_SIZE):
		for y in range(start_y, start_y + CHUNK_SIZE):
			tile_map.set_cell(Vector2i(x, y), -1)


func generate_chunk(chunk_coord: Vector2i) -> void:
	generated_chunks[chunk_coord] = true

	var start_x = chunk_coord.x * CHUNK_SIZE
	var start_y = chunk_coord.y * CHUNK_SIZE

	# Increase difficulty and item rarity with height
	height_penalty = move_toward(
		ISLAND_THRESHOLD, 0.0, player_node.global_position.y * HEIGHT_PENALTY
	)

	var drop_ore = log((height_penalty * ORE_CURVE_STEEP_THRESH) + 1.0)
	ore_threshold = clamp(ORE_INIT_THRESHOLD - drop_ore, ORE_MIN_THRESHOLD, ORE_INIT_THRESHOLD)

	var drop_gem = log((height_penalty * GEM_CURVE_STEEP_THRESH) + 1.0)
	gem_threshold = clamp(GEM_INIT_THRESHOLD - drop_gem, GEM_MIN_THRESHOLD, GEM_INIT_THRESHOLD)

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

			if island_val < height_penalty:
				continue

			var block_type = STONE

			# Apply voids and ore only to the underground stone layer
			var void_noise_val = void_noise.get_noise_2d(x, y)
			var ore_noise_val = ore_noise.get_noise_2d(x, y)
			var gem_noise_val = gem_noise.get_noise_2d(x, y)

			if void_noise_val > EMPTY_CELLS_THRESHOLD:
				block_type = EMPTY_CELL
			elif ore_noise_val < DIRT_THRESHOLD:
				block_type = DIRT
			elif ore_noise_val > ore_threshold:
				block_type = ORE
			elif gem_noise_val > gem_threshold:
				block_type = GEM

			tile_map.set_cell(grid_position, TILE_SOURCE_ID, block_type)


func mine_tile(world_position: Vector2) -> bool:
	return terrain_generator.mine_tile(world_position)
