# Quill

**Quill** is a UI framework for the Bevy game engine. It's meant to provide a simple API for
constructing reactive user interfaces, similar to frameworks like React and Solid, but built on a
foundation of Bevy ECS state management.

Currently in "proof of concept" phase. This means that nothing is set in stone yet.

## Getting started

For now, you can run the example:

```sh
cargo run --example simple
cargo run --example nested
```

When the window opens, hit the spacebar to update the counter.

## Aspirations / guiding principles:

* Allows easy composition and re-use of hierarchical widgets.
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

## Architecture and Rendering Lifecycle

### Display Trees and View Trees

Quill maintains several different hierarchical structures:

The **display tree** is the tree of Bevy Ui Node entities which actually render. These are the
nodes which actually produce rendering commands which are sent to the GPU. In Quill, the display
tree is analogous to the HTML DOM: it's the output of a template or generator.

The **view tree** is the tree of nodes that generate the display tree. It's made up of `View` trait
objects. The most important method of a view is the `.build()` method, which is what actually
generates the nodes of the display tree. Views are able to differentially 'patch' the display
tree, modifying it in place instead of replacing it.

The display tree and view tree have similar hierarchical structure, but they are not the same.
Most view nodes generate a single display node, and most view nodes with children will generate
a display node with the same number of children. However, there are exeptions: A `For` node
will generate multiple children depending on the length of the array used as input, and conditional
nodes will generate a single child out of multiple possible options.

**Presenters** are Rust functions which generate a view tree; the return type of a presenter is
`impl View`. (If you have used React, Solid, or other similar frameworks, these are called
"component functions", however the name "component" means something different in Bevy.)

> [!NOTE]
> Note: The name "presenter" has nothing to do with the "Model/View/Presenter" design pattern. (Well,
> almost nothing.)

Here's an example of a basic presenter which creates an element with two children:

```rust
fn hello_world(mut cx: Cx) -> impl View {
    // `Element` is a generic UI node, kind of like an HTML "div".
    Element::new((
        "Hello, ", // Yes, raw string slices implement `View` too!
        Element::new("World!"),
    ))
}
```

A presenter is typically responsible for a small subset of the total view tree for the whole UI.
The `View` nodes output by a presenter fall into one of several types:

* "Built-in" view types, such as `Element`, `For`, `If` and so on. These are Rust objects which
  directly implement the `View` trait.
* Primitive types which implement `View`, such as `String` and `&str`.
* View nodes which invoke another presenter function.

For those who are familiar with React, the built-in views correspond to "intrinsic" types such as
`<div>` or `<button>`, where the presenter nodes correspond to components such as `<MyComponent>`.
However, the convention of using upper-case/lower-case is reversed here: Built-in views generally
start with an upper-case letter (because they are Rust structs), whereas presenters start with
a lower-case letter (because they are Rust functions).

`View` trees are stateless and immutable: each rendering cycle, a new `View` tree is constructed.
However, this is actually very cheap, because the output of a single presenter is not a tree of
allocated/boxed nodes in memory, but a set of nested tuples - in other words, it's a single
object stored in continguous memory. The only exception is when a presenter invokes another
presenter - in this case, the output of the nested presenter is boxed and stored in an ECS
component called a `ViewHandle`, which contains a type-erased reference to the output of the
presenter.

### Managing State

Because `View`s are stateless, their state must be managed externally. Each `View` has an associated
type, `View::State` which defines the type of the view's state. For most views, the `State` not only
includes the state for itself, but the state for all child views as well. This means that the state
object, like the view object, is a set of nested tuples stored in a single contiguous memory region.
View states, like `View`s, are also stored in the `ViewHandle`, however unlike the view tree the
state is mutable. The `.build()` method is responsible for updating the state at the same time
as it generates the display tree nodes.

A bit more about `ViewHandle`s: The handle contains everything needed to re-render a presenter,
including a reference to the presenter itself, it's arguments, the previous view tree (from the
last time the presenter was called) and the view state. This means that it's possible to regenerate
just a subset of the display tree without having to regenerate the whole thing, so long as you
have a reference to the `ViewHandle`.

`ViewHandles` are ECS components which are attached to the entities that make up the view tree.
Even though the output of a single presenter is a single `View` object, there are multiple
presenters (and in some cases a presenter may be invoked more than once), and each presenter
invocation is represented by an entity with a `ViewHandle`. Other ECS components are
also attached to this entity. The most important of these are components that handle reactivity.

To create a Quill UI, all you need to do is insert a `ViewHandle` into the ECS world. There can
be multiple `ViewHandles` for multiple independent UIs.

### Reactivity

"Reactive programming" is a development paradigm in which echews explicit subscribing and
unsubscribing from event sources. Instead the mere act of accessing data creates a dependency on it.
This dependency causes the using code to be re-run when the data changes. An easy analogy to
understanding this concept is a spreadsheet cell: when a formula has a reference to cells A1
and B2, the spreadsheet's internal engine knows when that cell needs to be updated (whenever A1
or B2 changes), there's no need to explicitly subscribe to them.

During rendering, presenters can invoke "reactive" functions such as `use_resource()`. These
functions do two things: First, they return the data that was requested, such as a resource.
Secondly, they add a "tracking" component to the view handle entity that indicates that the
`ViewHandle` and it's presenter has a dependency on that data, so that when that data changes,
the `ViewHandle` is re-rendered.

Quill contains an ECS system which queries these tracking components and re-renders the views which
are out of date. Note that tracking components are always cleared before calling the presenter,
because the presenter is expected to re-subscribe to its dependencies as a side-effect of execution.
This is how reactive frameworks like React and Solid work, and it's how we can get away with
not having to explicitly unsubscribe from our dependencies.

## Example

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