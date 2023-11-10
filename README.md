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

# Example

```rust
/// Define some styles
lazy_static! {
    static ref STYLE_MAIN: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .position(ui::PositionType::Absolute)
        .left(10.)
        .top(10.)
        .bottom(20.)
        .right(10.)
        .border(1)
        .border_color(Some(Color::hex("#888").unwrap()))
        .display(ui::Display::Flex)));
    static ref STYLE_ASIDE: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#222").unwrap()))
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .width(200)));
}

/// Function to set up the view root
fn setup_view_root(mut commands: Commands) {
    commands.spawn((TrackedResources::default(), ViewHandle::new(ui_main, ())));
}

/// Top-level presenter
fn ui_main(mut cx: Cx) -> impl View {
    let counter = cx.use_resource::<Counter>();
    // Render an element with children
    Element::new((
        Element::new(()).styled(STYLE_ASIDE.clone()),
        v_splitter,
        // A conditional element
        If::new(
            counter.count & 1 == 0,
            // Strings and string slices also implement `View`.
            "even",
            "odd",
        ),
    ))
    .styled(STYLE_MAIN.clone())
}

/// A presenter function
fn v_splitter(mut _cx: Cx) -> impl View {
    Element::new(Element::new(()).styled(STYLE_VSPLITTER_INNER.clone()))
        .styled(STYLE_VSPLITTER.clone())
}

```

# Styling

Quill supports CSS-like styling in the form of `StyleSet`s. A `StyleSet` is a sharable object
that contains a number of style properties like `background_color`, `flex_direction` and so on.
`StyleSet`s can be composed - that is, multiple `StyleSets`s can be applied to the same element,
and the resulting style is computed by merging all the style properties together. There is no
"cascade" as in CSS, styles are applied in the order they are declared.

Styles must be wrapped in an `Arc` because they are designed to be shared. Most styles are global
constants, but nothing prevents you from creating a style dynamically in your presenter function.

Styles are applied to an element using the `.styled()` method, which accepts either a single style,
or a tuple of styles.

`StyleSet`s are typically creating using the `.build()` method, which accepts a closure that takes
a builder object. The builder methods are flexible in the type of arguments they accept: for
example, methods such as `.margin_right()` and `.row_gap()` accept an `impl Length`, which can be
an integer (i32), a float (f32), or a Bevy `ui::Val` object. In the case where no unit is specified,
pixels is the default unit, so for example `.border(2)` specifies a border width of 2 pixels.

**Coming Soon**: Dynamic selectors, classes, and CSS variables.

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