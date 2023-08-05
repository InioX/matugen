# Where should we write the GIF?
Output colorscheme.gif

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
Type "matugen color hex '#b3cdb0' -p 'triadic'"
Sleep 500ms
Enter

# Clear the screen
Sleep 2s
Ctrl+L

# Hsl - adjacent
Type "matugen color rgb 'rgb(179,205,176)' -p "adjacent""
Sleep 500ms
Enter

Sleep 5s