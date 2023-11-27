# TODO:

* Work on cargo docs.
* Some kind of sugar for stylesets that avoids all the .clone() calls.
* Change style builder methods to accept `impl StyleValue`. Then define `Var("varname")`
* [test] Leaf nodes render when dependencies change (currently render unconditionally).
* [test] Detect changes to presenter props.
* `use` hooks for components, events, etc.
* Implement button click handler. (Right now using an event, but closure would be better).
* Basic widgets: Button, Slider, Text Input.
* Support for `cursor` style property.
* Change QuillPlugin to add bevy_mod_picking plugins if needed:

    .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
