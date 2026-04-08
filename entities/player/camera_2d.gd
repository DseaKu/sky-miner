extends Camera2D

const DEFAULT_ZOOM := Vector2(2.75, 2.75)


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	zoom = DEFAULT_ZOOM


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("one"):
		zoom = Vector2(0.3, .3)
	if Input.is_action_just_pressed("two"):
		zoom = Vector2(1., 1.)
	if Input.is_action_just_pressed("three"):
		zoom = DEFAULT_ZOOM
