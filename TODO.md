# TODO:

* Some kind of sugar for stylesets that avoids all the Arc::new() and .clone() calls.
* Come up with a better way to do hover than listening for In/Out events.
* Change TrackedResources to automatically create the component when needed.
* Leaf nodes render when dependencies change (currently render unconditionally).
* Option for ViewHandle to parent to an explicit (manually constructed) UiNode.
* Experiment with dependency injection instead of hooks.
* Detect changes to presenter props.
* Local state for presenters (`use_state`).
* `use` hooks for components, events, etc.
* Misc cleanup and reorganization.
* Implement button click handler.
* Basic widgets: Button, Slider, Text Input.
* ElementContext shouldn't need to be mutable except for world - use a refcell?
* Copying of class name vectors during style computation is not very efficient.
* No-argument presenters don't store state in a ViewHandle. This is because of lifetime issues
  with No-argument presenters.
