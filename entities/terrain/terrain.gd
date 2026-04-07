
extends Node2D

@onready var tile_map = $TileMapLayer

const WIDTH = 100
const DEPTH = 50

const TILE_SOURCE_ID = 0
const DIRT_ATLAS_COORD = Vector2i(0, 0)
const STONE_ATLAS_COORD = Vector2i(1, 0)
const ORE_ATLAS_COORD = Vector2i(2, 0)
const EMPTY_CELLS_ATLAS_COORD = Vector2i(3, 3)

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
			var block_type = DIRT_ATLAS_COORD
			
			if y > 2:
				block_type = STONE_ATLAS_COORD
				
				var ore_noise_val = ore_noise.get_noise_2d(x, y)
				var void_noise_val = void_noise.get_noise_2d(x,y)

				if ore_noise_val > ORE_THRESHOLD: 
					block_type = ORE_ATLAS_COORD
				
				elif void_noise_val > EMPTY_CELLS_THRESHOLD:
					block_type = EMPTY_CELLS_ATLAS_COORD


			tile_map.set_cell(grid_position, TILE_SOURCE_ID, block_type)

func mine_tile(world_position: Vector2) -> bool:
	var local_pos = tile_map.to_local(world_position)
	var grid_pos = tile_map.local_to_map(local_pos)
	
	# Check if there is a tile there
	var source_id = tile_map.get_cell_source_id(grid_pos)
	if source_id != -1:
		var atlas_coords = tile_map.get_cell_atlas_coords(grid_pos)
		if atlas_coords == ORE_ATLAS_COORD:
			print("Mined Ore!")
		elif atlas_coords == STONE_ATLAS_COORD:
			print("Mined Stone.")
		elif atlas_coords == DIRT_ATLAS_COORD:
			print("Mined Dirt.")
		
		tile_map.set_cell(grid_pos, -1) # Remove the tile
		return true
	return false
