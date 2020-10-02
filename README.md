# Cube Scramblers

A collection of scrambling algorithms for various programming languages

So far I have

- Lua
  - used in a World of Warcraft addon
- Swift
  - used in my iOS app Last Layer on the app store 
  - <a href="https://apps.apple.com/us/app/last-layer/id619590498?itscg=30200&amp;itsct=apps_box" style="width: 170px; height: 170px; border-top-left-radius: 22%; border-top-right-radius: 22%; border-bottom-right-radius: 22%; border-bottom-left-radius: 22%; overflow: hidden; display: inline-block; vertical-align: middle;"><img src="https://is5-ssl.mzstatic.com/image/thumb/Purple124/v4/6a/56/bd/6a56bd6d-77b4-fdf4-9e95-8dbc78ce1e59/AppIcon-0-0-1x_U007emarketing-0-0-0-7-0-0-sRGB-0-0-0-GLES2_U002c0-512MB-85-220-0-0.png/540x540sr.jpg&h=e09432fdcbb1d87bc505d9203b56e92d" alt="Last Layer" style="width: 170px; height: 170px; border-top-left-radius: 22%; border-top-right-radius: 22%; border-bottom-right-radius: 22%; border-bottom-left-radius: 22%; overflow: hidden; display: inline-block; vertical-align: middle;"></a>
- Rust
  - built as a p.o.c. learning rust, was going to use it in a terminal cli app, but never finished building it.

They all follow the same general pattern:
- create the moves
- check the move with a previous move if available, to make sure they dont match
- more complicated checks for seeing if the move matches a twice previous move.
- output to 5 strings of 5 moves each


