# TODO:

* Some kind of sugar for stylesets that avoids all the Arc::new() and .clone() calls.
* Change style builder methods to accept `impl StyleValue`. Then define `Var("varname")`
* Change TrackedResources to automatically create the component when needed.
* Leaf nodes render when dependencies change (currently render unconditionally).
* Option for ViewHandle to parent to an explicit (manually constructed) UiNode.
* Experiment with dependency injection instead of hooks.
* Detect changes to presenter props.
* Local state for presenters (`use_state`).
* `use` hooks for components, events, etc.
* Implement button click handler. (Right now using an event, but closure would be better).
* Basic widgets: Button, Slider, Text Input.
* ElementContext shouldn't need to be mutable except for world - use a refcell?
* Support for `cursor` style property.
* No-argument presenters don't store state in a ViewHandle. This is because of lifetime issues
  with No-argument presenters.
