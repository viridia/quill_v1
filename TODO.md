# TODO:

* Some kind of sugar for stylesets that avoids all the .clone() calls.
* Change style builder methods to accept `impl StyleValue`. Then define `Var("varname")`
* Conditional class names.
* `use` hooks for components, events, etc.
* Implement callbacks.
* Widgets to do:
    * Tree View
    * Popup Menu
    * Gizmo
    * Text Input.
* Support for `cursor` style property.
* Change QuillPlugin to add bevy_mod_picking plugins if needed:

    .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
