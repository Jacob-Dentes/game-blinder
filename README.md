## Game Blinder
This is a simple app for creating challenging video game playthroughs.
It allows binding keyboard and mouse inputs to turning the app window black.
This allows, for example, turning the screen black each time the player moves or clicks.

## Platforms
I have only tested this app on Windows11. I have no idea if it will work on any other
platform.

## Asset credits
All assets used are under the MIT license from [Phosphor icons](https://github.com/phosphor-icons/core).

## Antivirus
As an app with a sometimes-seethrough window that listens to global inputs, this app is
understandably prone to being flagged by antivirus. The app only listens to registered
inputs and never connects to the internet. That being said, you should not download an
app that you do not trust. Read and build from source if you have the understanding, 
otherwise use at your own discretion.

## Settings
To choose which inputs trigger the screen going black and decide how long the screen should
be black for, you should place a 'config.txt' file in the same directory as the executable.
The text file should be formatted in the following way. The first line is a comma-separated
list of the inputs to trigger black screen. The second line is a decimal number for the
number of seconds the screen will stay black for after each input. Here is an example config.txt:

`w, a, s, d, leftbutton, leftshift, comma`

`2.0`

With this configuration, pressing any of keys in "WASD", left clicking, pressing left shift,
or hitting the comma key will cause the app window to go black for 2 seconds.

**Note:** Holding down keys will act like the key is pressed every frame. Holding down mouse
buttons only registers as the inital click.

### A comprehensive list of key representations
This section contains all accepted input strings for the config file. As a note, the "lsuper"
and "rsuper" keys are the platform dependent super keys (for example, windows keys for windows
and command keys for mac); they are aliased with "lwindows" and "lcommand", "rwindows" and "rcommand,"
respectively.

#### Keyboard keys
- a
- b 
- c 
- d 
- e 
- f
- g 
- h 
- i 
- j 
- k 
- l 
- m 
- n 
- o 
- p 
- q 
- r 
- s 
- t 
- u 
- v 
- w 
- x 
- y 
- z 
- 0 
- ) 
- 1 
- ! 
- 2 
- @ 
- 3 
- \# 
- 4 
- $ 
- 5 
- % 
- 6 
- ^ 
- 7 
- & 
- 8 
- \* 
- 9 
- ( 
- ` 
- ~ 
- / 
- ? 
- < 
- . 
- \> 
- \- 
- _ 
- ; 
- : 
- [ 
- { 
- ] 
- } 
- = 
- \+ 
- \ 
- | 
- ' 
- "
- backspace
- tab
- enter
- escape
- space
- pageup
- pagedown
- end
- home
- left
- up
- right
- down
- insert
- delete
- lsuper
- lwindows
- lcommand
- rsuper
- rwindows
- rcommand
- numpad0
- numpad1
- numpad2
- numpad3
- numpad4
- numpad5
- numpad6
- numpad7
- numpad8
- numpad9
- f1
- f2
- f3
- f4
- f5
- f6
- f7
- f8
- f9
- f10
- f11
- f12
- f13
- f14
- f15
- f16
- f17
- f18
- f19
- f20
- f21
- f22
- f23
- f24
- numlock
- scrolllock
- capslock
- leftshift
- rightshift
- leftcontrol
- rightcontrol
- leftalt
- rightalt
- back
- forward
- refresh
- volumemute
- volumedown
- volumeup
- medianext
- mediaprevious
- mediastop
- mediaplay
- backquote
- slash
- backslash
- comma
- period
- minus
- quotekey
- semicolon
- leftbracket
- rightbracket
- equal

#### Mouse buttons
- leftbutton
- rightbutton
- middlebutton
- x1button
- x2button
