# Cube Scramblers

A collection of scrambling algorithms for various programming languages

So far I have

- Lua
  - used in a World of Warcraft addon
- Swift
  - used in my iOS app Last Layer on the app store 
  - [Download it on the appstore](https://apps.apple.com/us/app/last-layer/id619590498?itscg=30200&amp;itsct=apps_box)
- Rust
  - built as a p.o.c. learning rust, was going to use it in a terminal cli app, but never finished building it.

They all follow the same general pattern:
- create the moves
- check the move with a previous move if available, to make sure they dont match
- more complicated checks for seeing if the move matches a twice previous move.
- output to 5 strings of 5 moves each


