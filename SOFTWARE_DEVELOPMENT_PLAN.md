# Software Development Plan for AVB

The game has to run on Windows and Linux natively.

## Technologies

- [Bevy](https://bevy.org): The game engine.
- [Lightyear](https://crates.io/crates/lightyear): The networking crate with inbuild entity replication.
- [Avian](https://crates.io/crates/avian3d): The physics crate.

### Utility

- [rayon](https://crates.io/crates/rayon): A data parallelism crate.
- [clap](https://crates.io/crates/clap): A command line argument parser.
- [log](https://crates.io/crates/log): A logging crate.
- [serde](https://crates.io/crates/serde): Its serde!
- [toml](https://crates.io/crates/toml): A en- and decoder for TOML.

### Maybe

- [meshem](https://crates.io/crates/bevy_meshem): A voxel to mesh crate.
- [bevy_water](https://crates.io/crates/bevy_water): A dynamic water material.

## Version Plan

### MVP (Minimal Vialble Product)

#### Systems

- Vehicle Editor
    - Support for 1x1x1 cm blocks
        - A list of parts that has a 1x1x1 block in it.
            - A central list of available parts.
        - Ability to pick a part from the list and build it after.
            - Way to know which part is currently selected.
    - Building blocks onto each other
        - Find out on which surface the mouse is pointing
    - Saving the vehicle
        - A list of saved vehicles
            - A way to save a new vehicle
            - Select a existing vehicle to overwrite
        - Save system
    - Loading a vehicle
        - A list of saved vehicles
        - Load system
    - Spawning the vehicle into the World
        - Building the vehicle into a mesh
        - Calculating weight, Center of mass
        - Spawning the mesh into the world
    - Multiplayer building
        - Way to pick which build session to join
        - Sync all actions to all players
    - Player movement
        - WASD for forward left backwards right
        - QE for down up
        - Shift for faster speed
        - STRG for slower speed
- Player movement
    - WASD for forward left backwards right
    - Space for jumping
    - Shift for running
    - STRG for crouching
    - ALT for laying down

#### Other

- A barren island
- Simple 1x1x1 cm cube