
# Question: Heat Map Population in Rust Implementation

Why populate_heat_map has not been implemented on the Rust side?

## Rust Background

- Currently unable to implement heat map population in Godot Engine
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

### Observations

- Function calls lead to unpredictable game crashes
- Occasional successful execution indicates non-deterministic behavior
- Integration between Rust-generated library and Godot Engine appears unstable

## Using signals in between GdScript and Rust

To establish a working implementation, a communication system has been designed using signal emissions between GDScript and Rust.

The chosen solution leverages signal emissions as the primary communication method between GDScript and Rust components.
