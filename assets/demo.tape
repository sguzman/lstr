# lstr demoo script for vhs

# 1. Setup the output and terminal properties
Output "assets/lstr-demo.gif"
#Set Width 1200
#Set Height 600
Set Width 1000
Set Height 618
Set FontSize 18
Set TypingSpeed 65ms
Set Theme "Catppuccin Frappe"

# 2. Show the classic view with icons
Type "lstr -g --icons"
Sleep 1s
Enter
Sleep 5s
Ctrl+L

# 3. Show the classic view with size and permissions
Type "lstr -Ggps --icons"
Sleep 1s
Enter
Sleep 5s
Ctrl+L

# 4. Show the interactive mode, pre-expanded
Type "lstr i -Ggps --icons --expand-level 2"
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
Down 6
Sleep 2s
Enter
Sleep 5s
Type ":x"
Enter
Sleep 2s
