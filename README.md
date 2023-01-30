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

## What we realized in the first part:
- creating block types - we implemented the way it should work, can be extended by changing graphics and using texture containers
- generating world maps - we implemented basic random map generation, can be extended by spawning resources in groups and by spawning caves. We also managed to create a wall map and prepare a space for building in the next part.
- adding a player and an option to move - after hours spent with colliders, we managed to add them to map and player, they do collide, unfortunately camera does not follow player, and player kinda flies. Extension options: make it work as intended.
- player is able to mine now - we managed to implement highlighting the tiles, the next steps are: highlight them in range of a player, and destroy tiles that are in the range of the player.

- After launching the game, using arrows you can steer our cool player, and using WASD, Z, X you can view the map (option for debug). Right now colliders projection can be seen, this is also a debug option.

## What we realized in the second part:
- improved map randomization
  - improved algorithm
  - added caves generator
- fixed colliders - they have proper size now and work almost as intended
- changed graphics - we changed the graphics to be more similar to the original game
- destructing the blocks
  - with animation
  - within player range
- implemented intended player movement
  - now we can move left, right
  - using space we can jump!
- camera follows the player



### How to run the game
- Just `cargo run`
- In case you need debug information:  
  `cargo build --features "debug" && cargo run --features "debug"`  
  Note that it may lead to decrease in app's performance.
- Now to steer our even cooler player use **A, D, SPACE** 
- We left 2 debug options:
  - [commented in main] colliders projection can be seen
  - [commented in main] camera movement with Arrows and Z,X for zoom 