extends Node2D

@onready var tile_map = $TileMapLayer

const WIDTH = 100
const DEPTH = 50

# Atlas Coords
const TILE_SOURCE_ID = 0
const DIRT = Vector2i(0, 0)
const STONE = Vector2i(1, 0)
const ORE = Vector2i(2, 0)
const EMPTY_CELL = Vector2i(3, 3)
const NONE_EXISTING_CELL = Vector2i(-1, -1)  # Defined by Godot

const ORE_SEED = 1
const ORE_SPREAD = 0.45
const ORE_THRESHOLD = 0.55
const EMPTY_CELLS_SEED = 2
const EMPTY_CELLS_SPREAD = 0.45
const EMPTY_CELLS_THRESHOLD = 0.3

# Define the noise generator
var ore_noise = FastNoiseLite.new()
var void_noise = FastNoiseLite.new()


func _ready() -> void:
	add_to_group("terrain")
	setup_noise()
	generate_terrain()


func setup_noise() -> void:
	ore_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	ore_noise.seed = ORE_SEED
	ore_noise.frequency = ORE_SPREAD

	void_noise.noise_type = FastNoiseLite.TYPE_SIMPLEX
	void_noise.seed = EMPTY_CELLS_SEED
	void_noise.frequency = EMPTY_CELLS_SPREAD


func generate_terrain() -> void:
	for x in range(WIDTH):
		for y in range(DEPTH):
			var grid_position = Vector2i(x, y + 5)
			var block_type = DIRT

			if y > 0:
				block_type = STONE

				var ore_noise_val = ore_noise.get_noise_2d(x, y)
				var void_noise_val = void_noise.get_noise_2d(x, y)

				if ore_noise_val > ORE_THRESHOLD:
					block_type = ORE

				elif void_noise_val > EMPTY_CELLS_THRESHOLD:
					block_type = EMPTY_CELL

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
