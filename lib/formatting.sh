#|---------------------------------------------------|
#| These: **    ** represent the area/object being   |
#| referenced in comments (only applies to comments) |
#|---------------------------------------------------|

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
### Background
BG_BLACK="40"
BG_RED="41"
BG_GREEN="42"
BG_YELLOW="43"
BG_BLUE="44"
BG_PURPLE="45"
BG_CYAN="46"
BG_WHITE="47"