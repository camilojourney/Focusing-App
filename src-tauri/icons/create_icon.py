# Simple script to create a basic template icon for testing
import struct

# Create a simple 18x18 PNG with a black circle on transparent background
width = height = 18
center_x = center_y = 9
radius = 7

# PNG file structure
png_data = b'\x89PNG\r\n\x1a\n'

# IHDR chunk
ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)  # RGBA
png_data += struct.pack('>I', 13) + b'IHDR' + ihdr
png_data += struct.pack('>I', 0xCRC)  # placeholder CRC

# Simple IDAT with black circle
pixels = []
for y in range(height):
    pixels.append(0)  # filter type
    for x in range(width):
        # Check if pixel is inside circle
        dx = x - center_x
        dy = y - center_y
        if dx*dx + dy*dy <= radius*radius:
            pixels.extend([0, 0, 0, 255])  # black, opaque
        else:
            pixels.extend([0, 0, 0, 0])  # transparent

print("Use Preview or another image editor to fix the icon manually")
print("The icon must have:")
print("1. Pure black (#000000) for visible parts")
print("2. Transparent (alpha=0) for invisible parts")
print("3. No anti-aliasing, no grays")
