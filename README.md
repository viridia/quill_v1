# Quill

**Quill** is a UI framework for the Bevy game engine. It's meant to provide a simple API for
constructing reactive user interfaces, similar to frameworks like React and Solid, but built on a
foundation of Bevy ECS state management.

Currently in "proof of concept" phase.

## Getting started

For now, you can run the example:

```sh
cargo run --example simple
cargo run --example nested
```

When the window opens, hit the spacebar to update the counter.

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

# Design Notes

Quill is an experimental library which borrows ideas from a number of popular UI frameworks,
including React.js, Solid.js, and Xilem. However, the way these ideas are implemented is quite
different, owing to the need to build on the foundations of Bevy ECS.

A Quill user inteface consists of a "view root", which represents the topmost element of the UI.
View roots are Bevy components, and multiple view roots are permitted.

The view root maintains two trees: the first is a "display" tree made up of Bevy UI elements
(UiNodes). This tree is used to produce the actual rendering commands that are issued to the GPU.

The second tree is the "view state" tree, which acts more like a template or a generator. The view
state builds the display tree, and patches it when reacting to changes in application state.
The view state tree also contains conditional logic such as If and For nodes.

The view state tree is produced by "presenters", which are callable functions that generate a
view state. The output of presenter is an object that implements the `View` trait.

(Note: the word "presenter" has nothing to do with the "Model / View / Presenter" design pattern.
Well, almost nothing.)

The `View` trait has several methods, but the most important one is `.build()`. This is the
method that actually builds the display graph. When called the first time, it will create the
UiNodes for the display graph. On subsequent calls, it will take the existing display graph and
update it, applying the minimum changes needed to bring it up to date with the current
view state.

Any object can implement `View`. For example, there are implementations of `View` for both
`String` and `&str`, which means that ordinary strings can be used as child nodes without the
need to wrap them in a special "text" element.

The view state graph is not necessarily long-lived: each time the presenter function is called, a
new view state graph is created, replacing the previous one. Fortunately, the view state graph is
all "inlined", meaning that the tree structure is made out of nested tuples rather than separately
allocated nodes. As a result, the view state graph for a presenter is a single stack-allocated
object, and re-constructing it is a fairly cheap operation.

Because the view state graph is constantly being recreated, it needs a way to preserve its state
across re-renders. This is handled by an associated type, `View::State`, which is state that
is external to the view state graph. Like the view state graph, it also consists of nested tuples,
but these are stored in an ECS component rather than on the stack, allowing them to persist between
render cycles.

Even though the view state graph is frequently reconstructed, it's "shape" is relatively stable,
unlike the display graph. For example, a `For` element may generate varying numbers of children
in the display graph, but each new iteration of the view state graph will have a `For` node in
the same relative location.

A helper class which is used by views is `NodeSpan`, which is kind of like a "rope" for Bevy
Entities. The `.build()` method of each `View` produces exactly one `NodeSpan`, however that span
may contain zero, one, or a varying number of entities that represent child nodes in the display
tree. `NodeSpan`s are also stored along with the view State in ECS components. This list of entities
is flattened before it is attached to the parent entity.

To illustrate how this works, consider the following example: Say a presenter produces a sequence
of three elements, where the second element is a "For" element. This means that the output of
`.build()` will produce three `NodeSpans`, but the middle `NodeSpan` will contain a varying number
of entities based on the data passed to the `For`. For a list of n items passed to `For`, the total
number of entities for the presenter will be n + 2. As the for loop reacts to changes in the length
of the array, it will always know where in the flat list of entities those changes will go.