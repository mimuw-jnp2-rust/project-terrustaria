# Terrustaria

## Authors
- Tomasz Głąb (@Toomimi on GitHub)
- Jan Kwiatkowski (@jk-89 on GitHub)

## Description
Terrustaria is going to be a simplified Rust implementation of well known game - Terraria.

## Features
- generating unique world maps
- moving and mining
- collecting items in equipment
- crafting new items
- building new structures
- different types of blocks (water, coal, copper etc.)
- optional - combat system

## Plan
First part:
- creating block types
- generating world maps
- adding a player and an option to move
- player is able to mine now

Second part:
- saving and loading a game
- main menu
- adding and removing items from the equipment
- building new structures from collected items
- crafting new items from collected ones
- combat system (optional)

## Libraries
- Bevy
- bevy_ecs_tilemap library (https://github.com/StarArawn/bevy_ecs_tilemap)
- Serde for serialization.
