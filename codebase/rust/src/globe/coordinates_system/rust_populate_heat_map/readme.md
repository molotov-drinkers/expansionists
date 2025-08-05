
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

-------------
## Gemini Analysis: Pure Rust Locks vs. Godot Dictionary for Heatmap State

Okay, let's theorize about the potential issues you faced when using the pure Rust `Arc` and `Mutex`/`RwLock` approach for your heat map compared to the Godot `Dictionary` workaround. Based on the code and your description in the README, here are some possible explanations:

### FFI (Foreign Function Interface) Complexity with Locks

* **The Core Issue:** Integrating Rust code with Godot involves crossing the FFI boundary via `gdext`. Managing synchronization primitives like `Mutex` or `RwLock` across this boundary can be tricky. While your vanilla Rust implementation worked fine in isolation, introducing Godot adds complexity.
* **Potential Problems:**
    * **Undefined Behavior:** Holding a Rust lock while calling back into Godot, or having Godot call into Rust code that immediately tries to acquire a lock held by another Godot-initiated call, might lead to undefined behavior or crashes depending on how `gdext` handles thread state and memory across the boundary.
    * **Deadlocks:** If different parts of your Godot logic (potentially running on different threads or call stacks) try to acquire locks on the same `Arc<Mutex<HashMap>>` or `Arc<RwLock<HashMap>>` in an inconsistent order, it could easily lead to deadlocks, freezing or crashing the game thread. This is a common issue in concurrent programming, exacerbated by the FFI layer.
* **`gdext` Marshalling:** The process `gdext` uses to convert (marshal) data types between Rust and Godot might not interact perfectly with Rust's locked data. There might be assumptions about data lifetimes or thread ownership that are violated when locks are involved.

### Concurrency and Godot's Threading Model

* **Godot's Approach:** Godot nodes and scripts often operate within specific threading contexts managed by the engine. While you can use threads, managing shared mutable state requires care.
* **Rust `Arc`/`Mutex`:** These are Rust's standard tools for safe multi-threaded shared mutable state. However, their correct use depends on understanding the exact threading model of the host (Godot, in this case) when called via FFI. Your README mentions suspected concurrency issues and that the `Arc` locks might not be working as expected within the game.
* **The Clash:** It's possible that the way Godot's engine calls into your Rust code (e.g., during physics processing for multiple `Troop` instances potentially calling `get_in_the_frontiers_trajectory`) creates contention or locking patterns that the Rust code, although safe in pure Rust, cannot handle gracefully when interwoven with Godot's execution model via `gdext`.

### Why Godot Dictionaries (via Metadata) Worked

* **Engine Management:** Godot's `Dictionary` type is a built-in engine type (a `Variant`). When you use `get_meta` and `set_meta`, you are interacting with Godot's own managed memory and potentially its internal mechanisms for handling concurrent access (or ensuring single-threaded access) to node metadata.
* **Simpler Interface:** This approach delegates the complexity of state management and potentially concurrency control (at least for the dictionary itself) to the Godot engine. You are essentially serializing/deserializing keys and values (String representations of coordinates and integer distances) into a Godot-managed container, bypassing the Rust-level lock management across the FFI.
* **FFI Friendliness:** Passing basic types like strings and integers, and interacting with Godot's `Variant` system via `get_meta`/`set_meta`, is a more straightforward and typically safer FFI interaction pattern than trying to share complex, locked Rust data structures directly.

### In Summary

The pure Rust `Arc<Mutex/RwLock<HashMap>>` likely failed due to the **complexities of managing Rust's concurrency primitives across the `gdext` FFI boundary within Godot's specific threading and execution model**. Issues like deadlocks, unexpected behavior during marshalling, or improper lock handling across threads could cause the crashes and instability you observed. The Godot `Dictionary` approach circumvented these issues by leveraging Godot's built-in, engine-managed data type and its associated access mechanisms (`get_meta`/`set_meta`), providing a simpler and more robust FFI interaction pattern for this specific use case.