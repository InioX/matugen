# Where should we write the GIF?
Output color.gif

# Set up a 1200x600 terminal with 46px font.
Set FontSize 46
Set Width 1200
Set Height 750

# Image - default
Type "matugen image assets/images/green.jpg -p 'default'"
Sleep 500ms
Enter

# Clear the screen
Sleep 2s
Ctrl+L

# Hex - triadic
Type "matugen color hex '#FFBF9B' -p 'triadic'"
Sleep 500ms
Enter

# Clear the screen
Sleep 2s
Ctrl+L

# Hsl - adjacent
Type "matugen color hsl 'hsl(300, 26.7%, 52.4%)' -p "adjacent""
Sleep 500ms
Enter

Sleep 5s