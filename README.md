# Quill

**Quill** is a UI framework for the Bevy game engine. It's meant to provide a simple API for
constructing reactive user interfaces, similar to frameworks like React and Solid, but built on a
foundation of Bevy ECS state management.

Currently in "proof of concept" phase.

## Aspirations / guiding principles:

* Allows easy composition and re-use of hierarchical components (called "presenters" to avoid
  confusion with Bevy ECS "components").
* No special syntax required, it's just Rust.
* Allows reactive hooks such as `use_resource()` that hook into Bevy's change detection framework.
* Built on top of existing Bevy UI components - presenters construct a graph and modify it in
  response to reactions.
* State management built on top of Bevy ECS, rather than maintaining its own separate UI "world".
* Any data type (String, int, color, etc.) can be displayed in the UI so long as it implements
  the `View` trait.
* Efficient rendering approach with minimal memory allocations. Uses a hybrid approach that borrows
  from both React and Solid to handle incremental modifications of the UI node graph.
* Supports CSS-like styling and dynamic visuals.

## Ideas borrowed from:

* Xilem
* React.js
* Solid.js
