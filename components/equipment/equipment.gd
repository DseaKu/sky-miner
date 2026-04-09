class_name Equipment
extends Node

enum Tool { NONE, PICKAXE, HARPOON, BINOCULARS }

func use_tool(tool: Tool) -> void:
	match tool:
		Tool.NONE:
			pass
		Tool.PICKAXE:
			pass
		Tool.HARPOON:
			pass
		Tool.BINOCULARS:
			pass
		_:
			pass
