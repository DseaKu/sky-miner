
extends Node2D

@onready var tile_map = $TileMapLayer

const WIDTH = 100
const DEPTH = 50

# These IDs correspond to your TileSet configuration.
# 0 is the ID of the texture atlas.
const TILE_SOURCE_ID = 0 
const DIRT_ATLAS_COORD = Vector2i(0, 0)
const STONE_ATLAS_COORD = Vector2i(1, 0)
const ORE_ATLAS_COORD = Vector2i(2, 0)

func _ready() -> void:
	generate_terrain()

func generate_terrain() -> void:
	for x in range(WIDTH):
		for y in range(DEPTH):
			# Offset the generation so it starts below the player
			var grid_position = Vector2i(x, y + 5) 
			
			var block_type = DIRT_ATLAS_COORD
			
			# Basic depth logic: Dirt on top, stone below
			if y > 5:
				block_type = STONE_ATLAS_COORD
				
				# Random chance for ore inside the stone layer
				if randf() < 0.05: # 5% chance
					block_type = ORE_ATLAS_COORD
					
			# Place the tile: set_cell(coords, source_id, atlas_coords)
			tile_map.set_cell(grid_position, TILE_SOURCE_ID, block_type)
