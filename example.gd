extends Node

# Example usage of BlueDot for Bluetooth LE communication

var bluedot: BlueDot
var devices: Array = []
var connected_device: BLEDevice = null

func _ready():
	# Create BlueDot instance
	bluedot = BlueDot.new()

	# Initialize Bluetooth LE
	if bluedot.initialize():
		print("BlueDot initialized successfully!")
		scan_for_devices()
	else:
		print("Failed to initialize BlueDot")

func scan_for_devices():
	print("Scanning for BLE devices...")

	# Scan for 5 seconds
	devices = bluedot.scan(5.0)

	print("Found %d devices:" % devices.size())
	for device in devices:
		print("  - %s (%s)" % [device.get_name(), device.get_address()])

	# Connect to first device (example)
	if devices.size() > 0:
		connect_to_device(devices[0])

func connect_to_device(device: BLEDevice):
	print("Connecting to %s..." % device.get_name())

	if device.connect():
		connected_device = device
		print("Connected!")

		# Discover and print services
		var services = device.get_services()
		print("Available services:")
		for service in services:
			print("  Service: %s" % service)
			var characteristics = device.get_characteristics(service)
			for characteristic in characteristics:
				print("    Characteristic: %s" % characteristic)

		# Example: Write to a characteristic
		# write_example()

		# Example: Read from a characteristic
		# read_example()
	else:
		print("Failed to connect")

func write_example():
	# Example UUIDs (replace with your device's UUIDs)
	var service_uuid = "0000180f-0000-1000-8000-00805f9b34fb"
	var char_uuid = "00002a19-0000-1000-8000-00805f9b34fb"

	# Create data to write
	var data = PackedByteArray([0x01, 0x02, 0x03])

	if connected_device.write(service_uuid, char_uuid, data):
		print("Write successful!")
	else:
		print("Write failed")

func read_example():
	# Example UUIDs (replace with your device's UUIDs)
	var service_uuid = "0000180f-0000-1000-8000-00805f9b34fb"
	var char_uuid = "00002a19-0000-1000-8000-00805f9b34fb"

	var data = connected_device.read(service_uuid, char_uuid)
	print("Read %d bytes: %s" % [data.size(), data])

func _exit_tree():
	# Disconnect when exiting
	if connected_device and connected_device.is_connected():
		connected_device.disconnect()
