extends Control

@export var steam_registry_path: String

enum SteamPathDetection {
	INVALID_REGISTRY_PATH = 0,
	INCORRECT_VALUE = 1,
	STEAM_NOT_FOUND = 2
}


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	print("Steam installation path:")
	print(await windows_get_steam_path())


# On Windows, return the Steam installation folder path
func windows_get_steam_path():
	var out = []
	await OS.execute("reg", ["query", steam_registry_path, "/v", "InstallPath"], out)
	
	var output = "".join(out)
	
	if len(output) == 0:
		return SteamPathDetection.STEAM_NOT_FOUND
	
	var info_line = ""
	for i in output.split("\r\n"):
		if "InstallPath" in i:
			info_line = i
			
	if len(info_line) == 0:
		return SteamPathDetection.INVALID_REGISTRY_PATH
	
	var regex_expr = RegEx.new()
	regex_expr.compile("[A-Z]:[\\/\\\\].*")
	
	var path = regex_expr.search(info_line)
	if path:
		return path.get_string()
	
	return SteamPathDetection.INCORRECT_VALUE
	

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass