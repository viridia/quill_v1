# TODO:

* Some kind of sugar for stylesets that avoids all the Arc::new() and .clone() calls.
* Change TrackedResources to automatically create when needed.
* Leaf nodes render when dependencies change (currently render unconditionally).
* Option for ViewHandle to parent to an explicit (manually constructed) UiNode.
* Detect changes to presenter props.
* Local state for presenters (`use_state`).
* `use` hooks for components, events, etc.
* Misc cleanup and reorganization.
* Basic widgets: Button, Slider, Text Input.
* ElementContext shouldn't need to be mutable except for world - use a refcell?
* Copying of class name vectors during style computation is not very efficient.
