# Quill

**Quill** is a UI framework for the Bevy game engine. It's meant to provide a simple API for
constructing reactive user interfaces, similar to frameworks like React and Solid, but built on a
foundation of Bevy ECS state management.

Quill is an experimental library which borrows ideas from a number of popular UI frameworks,
including React.js, Solid.js, Dioxus, and Xilem. However, the way these ideas are implemented is
quite different, owing to the need to build on the foundations of Bevy ECS.

## Getting started

For now, you can run the examples. The "complex" example shows off multiple features of the
library:

```sh
cargo run --example complex
```

## Aspirations / guiding principles:

* Allows easy composition and re-use of hierarchical widgets.
* Built on top of existing Bevy UI components.
* No special syntax required, it's just Rust.
* Allows reactive hooks such as `use_resource()` that hook into Bevy's change detection framework.
* State management built on top of Bevy ECS, rather than maintaining its own separate UI "world".
* Any data type (String, int, color, etc.) can be displayed in the UI so long as it implements
  the `View` trait.
* Efficient rendering approach with minimal memory allocations. Uses a hybrid approach that borrows
  from both React and Solid to handle incremental modifications of the UI node graph.
* Supports CSS-like styling and dynamic visuals.

Check out the demo video [here](https://youtu.be/NXabt3NrKMg).

## Examples usages

### A basic UI widget

The only requirements on a presenter is that it take a `Cx` context as its first argument, and return an `impl View` as its result.

```rust
// A presenter function
fn counter(mut cx: Cx<u8>) -> impl View {
    // Access data in a resource
    let counter = cx.use_resource::<Counter>();
    Element::new().children((
        format!("The count is: {}", counter.count),
    ))
}
```

### Conditional rendering with `If`

The `If` view takes a conditional expression, and two child views, one which is rendered when the condition is true, the other when the condition is false.

```rust
fn counter(mut cx: Cx<u8>) -> impl View {
    let counter = cx.use_resource::<Counter>();
    Element::new().children((
        "The count is: ",
        If::new(counter.count & 1 == 0, "even", "odd"),
    ))
}
```

### Rendering multiple items with `For`

`For::each()` takes a list of items, and a callback which renders a `View` for each item:

```rust
fn event_log(mut cx: Cx) -> impl View {
    let log = cx.use_resource::<ClickLog>();
    Element::new()
        .children(For::each(&log.0, |item| {
            Element::new()
                .styled(STYLE_LOG_ENTRY.clone())
                .children((item.to_owned(), "00:00:00"))
        })),
}
```

There is also `For::index()` and `For::keyed()`.

### Invoking child presenters

If a presenter takes no properties, then you can just use the name of the function directly.

For presenters which take properties, most of the time this will be a struct - but it doesn't have to be. Use the `.bind()` method to associate
a presenter with a set of property values.

```rust
fn root_presenter(mut _cx: Cx) -> impl View {
    Element::new().children((no_args, with_args.bind("Fred")))
}

fn no_args(mut cx: Cx) -> impl View {
    "I have no args"
}

fn with_args(mut cx: Cx<&str>) -> impl View {
    format!("I have one arg: {}", cx.props.name)
}
```

### Modifying the generated UI nodes

The `.with()` method takes a callback which allows you to directly modify the Bevy UI node:

```rust
fn event_log(mut cx: Cx) -> impl View {
    let log = cx.use_resource::<ClickLog>();
    Element::new()
        .with(|entity, world| {
            // Do stuff with the entity
        })),
}
```

`.with()` is called whenever the view is updated. There's another variant, `.with_memo()`, which
takes an additional argument representing a set of dependency values (which can be anything).
The callback will only be called when one or more of the following is true:

* This is the first time the view is being built.
* The display entity it's being called on is different from the previous update
  (this can happen if the underlying view rendered a different entity).
* The list of dependencies is different than the previous update.

```rust
fn selectable_widget(mut cx: Cx<SelectableWidgetProps>) -> impl View {
    let selected = cx.props.selected;
    Element::new()
        .with_memo(|entity, world| {
            // Modify the component, but only when `selected` changes.
            let mut cmp = entity.get_mut::<MyComponent>.unwrap();
            cmp.selected = selected;
        }, selected)),
}
```

There's also a shortcut method that lets you insert ECS bundles:

```rust
Element::new().insert((
    ViewportInsetElement {},
    On::Pointer<Move>::run(callback),
)),
```

### Returning multiple nodes

Normally a `View` renders a single UI node. If you want to return multiple nodes, use a `Fragment`:

```rust
fn fragment_example(mut cx: Cx) -> impl View {
    Fragment::new((
        "Hello, ",
        "World!"
    ))
}
```
The children of the `Fragment` will be inserted inline in place of the `Fragment` node.

### Atoms: Local state

It's common in UI code where a parent widget will have to keep track of some local state.
Often this state needs to be accessible by both the rendering code (the presenter) and the event
handlers. "Atoms" are a way to manage local state in a reactive way.

An `AtomHandle` is an identifier which can be used to get or set the value of an atom. You
can create a handle in one of several ways:

* `World::create_atom::<T>()` - returns a new atom handle from the World.
* `Cx::create_atom::<T>()` - returns a new atom handle from the current presenter context.
* `Cx::create_atom_init::<T>(init: fn() -> T)` - returns a new atom handle from the current presenter context and initializes it.

Handles created from `Cx` are automatically deleted when the presenter state is despawned.
Handles created on the world are not; you are responsible for deleting them.

Handles implement Copy and Clone, and can be captured by closures or passed as props to children.

There are several ways to access data in an atom. In `Cx`, there are get and set methods:

* `Cx::get_atom(handle)`
* `Cx::set_atom(handle, value)`

These functions are reactive: getting the value of an atom automatically adds the atom to the
current reaction tracking list, so the presenter will be re-run when the atom changes.

Another way to access atom values is via `AtomStore`, which is an injectable value:

* `AtomStore::get(handle)`
* `AtomStore::set(handle, value)`
* `AtomStore::update(handle, update_fn)`

`AtomStore` is intended to be used in non-reactive systems and event handlers, that is, functions
which don't have tracking contexts. Calling `.get()` does not track the atom. However, calling
`.set()` will trigger reactions in any other tracking contexts that depend on the atom.

```rust
fn atom_example(mut cx: Cx<&str>) -> impl View {
    let name = *cx.props;
    let counter = cx.create_atom_init::<i32>(|| 0);
    Element::new()
        .children((
            format!("The count is: {}: {}", name, cx.read_atom(counter)),
        ))
        .insert(On::<Pointer<Click>>::run(
            move |_ev: Listener<Pointer<Click>>, atoms: AtomStore| {
                atoms.update(counter, |value| value + 1);
            },
        })
}
```

### RefElement and explicit entity ids

The typical way of updating the state of an element is by modifying the state and props of
a presenter, which causes the elements to be rebuilt and patched. However, there are a few
cases where you may want to directly modify a display element at the ECS level from another element,
bypassing the normal update process. In this case, it's desirable for an element to contain
the entity id of another element, so that it access it directly.

One use case for this is expand / collapse animations, where you need direct access to the entity
in order to measure it's natural size before beginning the animation.

To do this, we can pre-allocate an entity id via `cx.create_entity()`. This method creates
a stable entity id which is "owned" by the presenter state, meaning that when the presenter
is razed, all of the owned ids will be despawned as well. `create_entity()` is a hook, so it returns
the same entity id each time the presenter function is run.

Now that we have an entity id, we can do two things with it:

* Render a `View` using that entity id, via `RefElement`.
* Pass that entity as a parameter to other `Views`, either as a property or as an attribute of
  a component. Those views can interrogate the components of that entity in the normal way.

`RefElement` is a `View` type, similar to `Element`. The only difference is that where `Element`
automatically spawns a new entity when building the `View`, `RefElement` uses the entity id that
you pass into it.

> [!NOTE]
> Note: Those who are familiar with React.js will note that this is the exact inverse of React's
> `useRef()` hook, although it is used for the same purpose. The `useRef()` hook creates an empty
> placeholder which is filled in with the element reference during rendering, whereas
> `create_entity` lets us allocate an element id before rendering happens.

### Advanced hooks

There are several advanced hooks in the examples directory. These hooks are not part of Quill,
but will likely be included in "Egret" which is the planned headless widget library based on Quill.

#### `use_enter_exit()`

The `.use_enter_exit()` hook is useful for elements such as dialog boxes which do both an open
and close animation. The idea is that when a dialog box closes you don't want to destroy the display
graph until the closing animation is complete:

```rust
use super::enter_exit::EnterExitApi;

let state = cx.use_enter_exit(open, 0.3);
```
In this example, `open` is a boolean flag which is true when we want the dialog to be open, and
false when we want it to be closed. Changing this flag starts a timer which drives a state machine;
the second argument is the delay time, in seconds, for the animations.

The output, `state`, is an enum which can have one of six values:

```rust
#[derive(Default, Clone, PartialEq)]
pub enum EnterExitState {
    EnterStart,
    Entering,
    Entered,
    ExitStart,
    Exiting,

    #[default]
    Exited,
}
```
These values can be converted into strings by calling `.as_class_name()` on them. The resulting
value can be put directly on the element as a class name, and the class names can be used in
dynamic stylesheet selectors.

The calling presenter should, in most cases, render the item whenever the state is not `Exited`.

```rust
let state = cx.use_enter_exit(open, 0.3);
If::new(
    state != EnterExitState::Exited,
    Element::new().class_name(state.as_class_name()), // Dialog content, etc.
    ()
```

#### `use_element_rect()`

The `use_element_rect()` hook returns the rectangular bounds of a ui node as a reactive data source,
meaning that it will trigger an update whenever the position or size of the element changes.
The input is the entity id of the element we wish to measure:

```rust
let rect = cx.use_element_rect(id_inner);
```

### Styling

#### Philosophy

There are several different ways to approach styling in Bevy. One is "inline styles", meaning that
you explicitly create style components (`BackgroundColor`, `Outline` and so on) in the template
and pass them in as parameters to the presenter.

A disadvantage of this approach is that you have limited ability to compose styles from different
sources. Rust has one mechanism for inheriting struct values from another struct, which is the
`..` syntax; this supposes that both of the struct values are known at the point of declaration.

Another disadvantage is that any dynamic style properties are strictly the responsibility of the
presenter. Transitory state changes such as "hover" and "focus" require updating the view and
patching the display graph. This means that widgets can only have whatever dynamic properties
are implemented in the presenter; it's not possible for an artist to come along later and add
hover or focus effects unless the widget is designed with hover effects in mind, and if the widget
has multiple parts, only the parts which have explicit support for those effects can be dynamically
styled.

This also means that, since presenters are constructed within an exclusive system, all dynamic
style changes must also happen in an exclusive system. While this will no doubt be true for most
state changes anyway ("drag" and "select" have to be done by the presenter), autonomous states
like "hover" and "focus" ought to be concurrently computable in a separate ECS system.

An alternative to inline styles is stylesheets or "style handles", which is a rule-based approach.
This has a number of advantages, but requires additional computation.

Quill's style system is inspired by CSS, but it is not CSS. Styles are currently built either
as constants, using a fluent syntax, or dynamically inline. A future addition should allow styles
to be loaded from assets, using a CSS-like syntax, and in fact the data representation of styles
has been carefully designed to allow for future serialization. Right now, however, the main focus
is on "editor" use cases, which likely will want styles defined in code anyway.

`StyleHandles` resemble CSS in the following ways:

* Style attributes are sparsely represented, meaning that only those properties that you actually
  declare are stored in the style handle.
* Styles are composable, meaning that you can "merge" multiple styles together to produce a union
  of all of them.
* Styles support both "long-form" and "shortcut" syntax variations. For example, the following are
  all equivalent:
  * `.border(ui::UiRect::all(ui::Val::Px(10.)))` -- a border of 10px on all sides
  * `.border(ui::Val::Px(10.))` -- Scalar is automatically converted to a rect
  * `.border(10.)` -- `Px` is assumed to be the default unit
  * `.border(10)` -- Integers are automatically converted to f32 type.
* Styles allow dynamism by defining "selectors", dynamic matching rules. These rules execute
  in their own dedicated ECS system, and use `Commands` to update the entity's style components.
* **Planned**: Styles will support inheritance of variables defined higher in the display graph.
  This will allow things like contextual color schemes ("light" and "dark" modes).

However, they also differ from CSS in a number of important ways:

* There is no prioritization or cascade, as this tends to be a source of confusion for web
  developers (Even CSS itself is moving away from this with the new "CSS layers" feature.) Instead
  styles are merged strictly in the order that they appear on the element.
* The syntax for selectors is limited, and certain CSS features which are (a) expensive to compute
  and (b) not needed for widget development have been left out.
* Styles can only affect the element they are assigned to, not their children. Styles can query
  the state of parent elements, but cannot affect them. This idea is borrowed from
  some popular CSS-in-JS frameworks, which have similar restrictions. The idea is to increase
  maintainability by making styles more deterministic and predictable.

#### Using StyleHandles

`StyleHandle`s are typically created using the `.build()` method, which accepts a closure that takes
a builder object. The builder methods are flexible in the type of arguments they accept: for
example, methods such as `.margin_right()` and `.row_gap()` accept an `impl Length`, which can be
an integer (i32), a float (f32), or a Bevy `ui::Val` object. In the case where no unit is specified,
pixels is the default unit, so for example `.border(2)` specifies a border width of 2 pixels.

Styles are applied to an element using the `.styled()` method, which accepts either a single style,
or a tuple of styles.

Here's an example of a widget which changes its border color when hovered:

```rust
use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use static_init::dynamic;

#[dynamic]
static STYLE_HOVERABLE: StyleHandle = StyleHandle::build(|ss| {
    ss.border_color("#383838")
        .border(1)
        .selector(":hover", |ss| {
            ss.border_color("#444")
        })
});

pub fn hoverable(cx: Cx) -> impl View {
    Element::new()
        .styled(STYLE_HOVERABLE.clone())
        .children(cx.props.children.clone())
}
```

An element can have multiple styles. Styles are applied in order, first-come, first-serve.

Conditional styles can be added via selectors. It supports a limited subset of CSS syntax (basically the parts of CSS that don't require backtracking):

* `:hover`
* `.classname`
* `:first-child` and `:last-child`
* `>` (parent combinator, e.g. `:hover > &`)
* `&` (current element)
* `,` (logical-or)
* **Planned:** `:focused`, `:focus-within`, `:not`.

As stated previously, selectors only support styling the *current* node - that is, the node that
the style handle is attached to. Selectors can't affect child nodes - they need to have their own styles.

So for example, `".bg:hover > &"` is a valid selector expression, but `"&:hover > .bg"` is not valid.
The `&` must always be on the last term. The reason for this is performance - Quill only supports those features of CSS that are lightning-fast.

#### Animated Transitions

Quill StyleHandles support CSS-like transitions for some properties (mostly layout properties
like width, height, left and so on, as well as transform properties like scale and rotation.
Eventually color once we get lerping figured out.)

The `transition` style attribute indicates which properties you want to be animated. Here's an
example of how to animate a rotation:

```rust
#[dynamic]
static STYLE_DISCLOSURE_TRIANGLE: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .transition(&vec![Transition {
            property: TransitionProperty::Transform,
            duration: 0.3,
            timing: timing::EASE_IN_OUT,
            ..default()
        }])
        .selector(".expanded", |ss| ss.rotation(PI / 2.))
});
```
How this works: when the styling system sees that a particular property is to be animated,
instead of modifying that style attribute directly, it injects an animation component that
contains a timer and an easing function. A separate ECS system updates the timer clock and
adjusts the style attribute.

Easing functions are just functions, so you can define whatever kind of easing you want.

### Class names

The `class_names` method can add class names to an element. Class names can be added conditionally
using the `.if_true()` modifier.

```rust
pub fn classnames_example(cx: Cx<Props>) -> impl View {
    Element::new()
        .class_names(("vertical", "selected".if_true(cx.props.selected)))
}
```

### Adding new hooks

In the example source file [./examples/complex/enter_exit.rs](./examples/complex/enter_exit.rs)
you'll find an example of how to add a custom hook method to the presenter context `Cx`.

The example implements (among other things) a `cx.use_enter_exit()` method, which creates
a state machine that is helpful for implementing animated open/close transitions. These kinds
of transitions are often seen in modal dialogs, popup menus, and so on.

## Architecture and Rendering Lifecycle

A Quill UI is made up of individual elements called `Views`. If you are familiar with web frameworks
like React.js, Solid.js or Vue, you'll recognizes that Quill views are like "components" or
"widgets": modular, resable elements that are arranged hierarchically. However, `Views` are not the
same as Bevy UI nodes; instead `Views` are templates which produce UI nodes.

Any object can implement `View`. For example, there are implementations of `View` for both
`String` and `&str`, which means that ordinary strings can be used as child nodes without the
need to wrap them in a special "text" element.

Views fall into two categories: built-in views, like `Element`, and user-created views. User-created
views are created by user functions written in Rust, which are called "presenters".

> [!NOTE]
> Note: The name "presenter" has nothing to do with the "Model/View/Presenter" design pattern. (Well,
> almost nothing.)

Presenter functions can depend on external data sources such as resources or state variables.
When these data sources are updated, the presenter function is run again, generating a new `View`.
The `View`, in turn, creates or modifies the Bevy UI nodes that make up the actual UI. Most of the
time, the Bevy UI nodes will be modified in place rather than being generated anew.

Here's an example of a basic presenter which creates an element with two children:

```rust
fn hello_world(mut cx: Cx) -> impl View {
    // `Element` is a generic UI node, kind of like an HTML "div".
    Element::new()
        .children((
            "Hello, ", // Yes, raw string slices implement `View` too!
            Element::new("World!"),
        ))
}
```

This examples shows a presenter function which returns a built-in `Element` view. It also
has two children, one of which is another `Element`, and one which is a string slice (`&str`).
Because Strings and string slices implement `View` they can be used anywhere a view can be.

When a UI is no longer needed (such as when a dialog or menu is closed), the view is *razed*
(the opposite of built), causing the various UI entities to be despawned.

The next sections describe this process in more detail.

### Display Trees and View Trees

The **display tree** is the tree of Bevy Ui Node entities. These are the nodes which actually
produce rendering commands which are sent to the GPU. In Quill, the display tree is analogous
to the HTML DOM: it's the output of a `View`.

The **view tree** is the tree of `View`s that generate the display tree. `View`s are trait
objects that know how to build and patch the display tree. Views contain a number of methods
for mantaining the display tree:

* `.build()` - initializes the nodes of the display graph.
* `.update()` - react to changes in the environment by modifying the nodes of the display graph.
* `.assemble()` - link together the nodes of the display graph in parent/child relationships.
* `.raze()` - disconnect and despawn any nodes generated by this view.

The display tree and view tree have similar hierarchical structure, but they are not the same.
Most view nodes generate a single display node, and most view nodes with children will generate
a display node with the same number of children. However, there are exeptions: A `For` node
will generate multiple children depending on the length of the array used as input, and conditional
nodes will generate a single child out of multiple possible options.

As an example, an `If` node has a true branch and a false branch, but only one branch can be built
at a time. When the conditional expression changes from `true` to `false`, the children generated
by the true branch are razed, and in their place the children generated by the false branch are
built.

The view tree is really a "tree of trees": that is, there is a larger tree whose nodes are made up
of `PresenterState`s, and each of those `PresenterState` nodes contains a tree of all the `Views`
generated by that presenter function. If a presenter calls another presenter, then the `View` nodes
of the parent `PresenterState` will contain links to the child `PresenterState`.

`PresenterState`s are what subscribes to reactive data sources and are the "unit of update
granularity"; it is not possible to update individual `View`s in isolation, instead the
entire `PresenterState` is updated together, with all of the `View`s within it. (This is closer
to the way React works than Solid does.) A `PresenterState` contains everything needed to
regenerate the views, which means that they can be updated in isolation, even if they are leaf
nodes or interior nodes of the view graph.

This diagram shows the relationship between presenters, `PresenterStates`, `Views`, and display nodes:

![View Tree](doc/ViewTree.drawio.png)

For those who are familiar with React, built-in views correspond to "intrinsic" types such as
`<div>` or `<button>`, whereas presenter functions correspond to function components such as
`<MyComponent>`. However, the convention of using upper-case/lower-case is reversed here: Built-in
views generally start with an upper-case letter (because they are Rust structs), whereas presenters
start with a lower-case letter (because they are Rust functions).

### Managing State

`View`s are stateless and immutable: each rendering cycle, a new `View` tree is constructed.
However, this is actually very cheap, because the output of a single presenter is not a tree of
allocated/boxed nodes in memory, but a set of nested tuples - in other words, it's a single
object stored in continguous memory.

Because `View`s are stateless, their state must be managed externally. Each `View` has an associated
type, `View::State` which defines the type of the view's state. For most views, the `State` not only
includes the state for itself, but the state for all child views as well. This means that the state
object, like the view object, is a set of nested tuples stored in a single contiguous memory
region. This is only true, however, for views that have a fixed number of children; for views
that have dynamic children, the view states are stored in a `Vec`.

View states, like `View`s, are also stored in the `PresenterState`, however unlike the view tree the
state is mutable. The `.build()` and `.update()` methods are responsible for updating the state at
the same time as it generates the display tree nodes. `PresenterStates` also keep a copy of the
parameters that were passed to the presenter function, and a copy of the last output.

`PresenterState`s are in turn stored inside an ECS component called a `ViewHandle`. The
`PresenterState` is type-erased (via `AnyPresenterState`). Thus, to maintain a reference to
the root of a UI, one only needs to keep track of the `ViewHandle` entity. You can have multiple
`ViewHandle`s for multiple independent UI displays.

Here's how to create a UI, given a root presenter:

```rust
commands.spawn(ViewHandle::new(ui_main, ()));
```

### Reactivity

"Reactive programming" is a development paradigm in which echews explicit subscribing and
unsubscribing from event sources. Instead the mere act of accessing data creates a dependency on it.
This dependency causes the using code to be re-run when the data changes. An easy analogy to
understanding this concept is a spreadsheet cell: when a formula has a reference to cells A1
and B2, the spreadsheet's internal engine knows when that cell needs to be updated (whenever A1
or B2 changes), there's no need to explicitly subscribe to them.

During rendering, presenters can invoke "reactive" functions such as `use_resource()`. These
functions do two things: First, they return the data that was requested, such as a resource.
Secondly, they add a "tracking" component to the `ViewHandle` entity that indicates that the
`ViewHandle` and it's presenter has a dependency on that data, so that when that data changes,
the `ViewHandle` is re-rendered.

Quill contains an ECS system which queries these tracking components and re-renders the views which
are out of date. Note that tracking components are always cleared before calling the presenter,
because the presenter is expected to re-subscribe to its dependencies as a side-effect of execution.
This is how reactive frameworks like React and Solid work, and it's how we can get away with
not having to explicitly unsubscribe from our dependencies.

### Memoization

`PresenterState` nodes are automatically memoized. This means that unless there is a change
to a dependency, or the props passed to the presenter change, then the presenter will not be
called again, the views will not be rebuilt, and the output display nodes will be the same as
from the previous render cycle.

A parent presenter can be re-rendered without re-rendering its children; similarly a child
presenter can be re-rendered without re-rendering its parent. If, however, the child node
produces a different entity than it did on the previous run, then the parent's display tree
will be updated to splice in the new child entity (this is what `.assemble()` does.).

Presenter props changes are detected by comparing the old prop values with the new. This means
that all props must implement `PartialEq`.

### Deep Dive: For-loops

`For` views are views that, given an array of data items, render a variable number of children.
There are three different flavors of `For` loops. The simplest, and least efficient, is the
`index()` loop. This loop simply renders each item at its index position in the array. The reason
this is inefficient is that the array may have insertions and deletions since the previous render
cycle. Thus, if element #2 becomes element #3, then the for loop will just blindly overwrite any
existing display nodes at position #3, destroying any nodes that don't match and building new
nodes in their place.

The next type is `.keyed()`, which is a bit smarter: it takes an additional function closure which
produces a unique key for each array element. The keys can be any data type, so long as they are
clonable and equals-comparable. The algorithm then attempts to match the old array nodes with the
new ones using an LCS (Longest Common Substring) matching algorithm. This means that as array
elements shift around, it will re-use the display nodes from the previous render, minimizing the
amount of churn. Any insertions or deletions will be detected, and the nodes in those positions
built or razed as appropriate.

Finally, there is `.each()`, which treats the actual array data as the key. This doesn't require
the extra closure argument, but requires that your array data implement `Clone` and `PartialEq`.

### Deep-Dive: NodeSpans

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

# Bibliography

* [Xilem: an architecture for UI in Rust](https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html)
* [Building a reactive library from scratch](https://dev.to/ryansolid/building-a-reactive-library-from-scratch-1i0p)
