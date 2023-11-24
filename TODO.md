# TODO:

* Work on cargo docs.
* Some kind of sugar for stylesets that avoids all the Arc::new() and .clone() calls.
* Change style builder methods to accept `impl StyleValue`. Then define `Var("varname")`
* Option for ViewHandle to parent to an explicit (manually constructed) UiNode.
* Experiment with dependency injection instead of hooks.
* [test] Leaf nodes render when dependencies change (currently render unconditionally).
* [test] Detect changes to presenter props.
* `use` hooks for components, events, etc.
* Implement button click handler. (Right now using an event, but closure would be better).
* Basic widgets: Button, Slider, Text Input.
* ElementContext shouldn't need to be mutable except for world - use a refcell?
* Support for `cursor` style property.
* Change QuillPlugin to add bevy_mod_picking plugins if needed:

    .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
