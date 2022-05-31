# Game Of Life got rusty

Hi! Here is my version of conway's game of life made in Rust using BevyEngine. It's not esthetically pleasing, and it lacks a few informations that could be beneficial for the study of this simple behaviour but it is planned to add them with newer releases.

My goal with this alpha was to learn ECS pattern as well as to try the bevy engine, as recommended by a teacher.

## Concept

Conway's game of life is as much of a game that it is a scientific tool about
Turing completeness and even physics support. The goal is to show that event with the simplest rules, you can make great complex systems that are Turing complete. The grid is now 250 cells by 250 cells, beyond this, it started to slow down a little.

## Controls

- `Left Click` to create a Cell
- `Right Click` to delete a Cell
- `Z` to go up on the canvas
- `Q` to go left
- `S` to go down
- `D` to go right
- `A` to zoom out
- `E` to zoom in

## What Next?

- [x] Base System
- [x] "Game" Loop
- [ ] Loop Counter
- [ ] Button Lock
- [ ] Button Color<->Game State
- [ ] GridSize Selection