@tool
extends EditorPlugin

func _enter_tree():
	print("BlueDot plugin activated")

func _exit_tree():
	print("BlueDot plugin deactivated")
