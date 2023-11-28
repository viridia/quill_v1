# TODO:

* Some kind of sugar for stylesets that avoids all the .clone() calls.
* Change style builder methods to accept `impl StyleValue`. Then define `Var("varname")`
* [test] Leaf nodes render when dependencies change (currently render unconditionally).
* [test] Detect changes to presenter props.
* `use` hooks for components, events, etc.
* Implement button click handler. (Right now using an event, but closure would be better).
* Basic widgets: Text Input.
* Support for `cursor` style property.
* Change QuillPlugin to add bevy_mod_picking plugins if needed:

    .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))

Notes on animation:

First, we need our traditional transition states: Entering, Entered, Exiting, Exited. These
need to be stored in a local.

Then we need to think about Bevy animation of UI. Do we want to support something like
CSS transitions? Or do we want to animate the global transform directly?
