
# Question: Virtual Planet Coordinate Map and Troop Heat Map as Rust Implementation

- Why hasn't the heat_map been implemented on the Rust side?
- Why do we need two implementations of Virtual Planet coordinate_map

## Rust Background

- Currently unable to implement heat map as hashMap on rust and use it on Godot Engine
- Existing implementation in `./vanila_path_finding.rs` successfully uses Rust-only solution
- The `get_in_the_frontiers_trajectory()` function returns `Vec<Vector3>` as expected

## Godot Engine Integration Issues

### Current Status

When implementing the Rust-generated library in the Expansionists game, we've encountered several issues:

- The game crashes upon function invocation
- Intermittent success in function execution
- Suspected concurrency issues within the gdext library

### Technical Details

The inconsistent behavior during function calls suggests potential race conditions or thread safety issues in the gdext library implementation.
Seems like the Arc locks might be not working.

### Observations

- Function calls lead to unpredictable game crashes
- Occasional successful execution indicates non-deterministic behavior
- Integration between Rust-generated library and Godot Engine appears unstable

## Using Godot Engine Dictionaries Metadata instead of Rust HashMap with ARCs

To establish a working implementation, a communication system has been designed using metadata creation for both VirtualPlanet (On Coordinate Map) and Troop (For HeatMap Trajectory Helper).

Thus, every interaction with these dictionaries, we call get_meta() function. To insert new data or to read it.

