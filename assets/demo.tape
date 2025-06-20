# lstr demo script for vhs

# 1. Setup the output and terminal properties
Output "assets/lstr-demo.gif"
#Set Width 1200
#Set Height 600
Set Width 1000
Set Height 618
Set FontSize 18
Set TypingSpeed 65ms
Set Theme "Catppuccin Frappe"
Set FontFamily "SpaceMono Nerd Font"

# 2. Show the classic view with icons
Type "lstr ../unifres -g -L=2 --icons"
Sleep 1s
Enter
Sleep 5s
Ctrl+L

# 3. Show the classic view with size and permissions
Type "lstr ../unifres -Ggps -L=2 --icons"
Sleep 1s
Enter
Sleep 5s
Ctrl+L

# 4. Show the interactive mode, pre-expanded
Type "lstr i ../unifres -Ggps --icons --expand-level=2"
Sleep 1s
Enter
Sleep 2s

# 5. Navigate and interact with the TUI
Down 7
Sleep 1s
Enter
Sleep 2s
Enter
Sleep 1s
Down 4
Sleep 2s
Enter
Sleep 5s
Type ":x"
Enter
Sleep 2s
