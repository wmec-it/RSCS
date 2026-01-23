#|---------------------------------------------------|
#| These: **    ** represent the area/object being   |
#| referenced in comments (only applies to comments) |
#|---------------------------------------------------|

#| Setting Variables |
TEST_VAR="TEST_VAR"
## Basic stuff needed
ESC="\e"           # USAGE **  \e  **[0;31mEscape Sequence**  \e  **[0m
CLOSE="[0m"        # USAGE \e[0;31mClosing Sequence\e**  [0m  **
SEP=";"            # USAGE \e[0**  ;  **31mFormat type separator\e[0m
F_END="m"          # USAGE \e[0;31**  m  **Colored text\e[0m
## Combine
START="$ESC["      # USAGE **  \e[  **0;31mStarting sequence\e[0m
END="$ESC$CLOSE"   # USAGE \e[0;31mEnd sequence**  \e[0m  **
## Colors
### Foreground
FG_BLACK="30"
FG_RED="31"
FG_GREEN="32"
FG_YELLOW="33"
FG_BLUE="34"
FG_PURPLE="35"
FG_CYAN="36"
FG_WHITE="37"
### Foreground High Intensity
FG_HI_BLACK="90"
FG_HI_RED="91"
FG_HI_GREEN="92"
FG_HI_YELLOW="93"
FG_HI_BLUE="94"
FG_HI_PURPLE="95"
FG_HI_CYAN="96"
FG_HI_WHITE="97"
### Background
BG_BLACK="40"
BG_RED="41"
BG_GREEN="42"
BG_YELLOW="43"
BG_BLUE="44"
BG_PURPLE="45"
BG_CYAN="46"
BG_WHITE="47"
### Background High Intensity
BG_HI_BLACK="100"
BG_HI_RED="101"
BG_HI_GREEN="102"
BG_HI_YELLOW="103"
BG_HI_BLUE="104"
BG_HI_PURPLE="105"
BG_HI_CYAN="106"
BG_HI_WHITE="107"
## Text Decorations
### Bold
BOLD="1"
### Underline
UNDERLINE="4"
### High Intensity
HIGH_INTENSITY="0"
BOLD_HIGH_INTENSITY="1"
### Italic
ITALIC="3"
BOLD_ITALIC="3m\e[1"
### Strikethrough
STRIKETHROUGH="9"