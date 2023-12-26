# TODO:

* Fix hanging bug
* :focus
* ViewTuple: Clone/PartialEq
* Migrate to Egret/Grackle:
    * Splitter
    * Disclosure
    * Dialog
    * Swatch
    * SwatchGrid
* `use` hooks for components, events, etc.
* Implement callbacks.
* Widgets to do:
    * Popup Menu
    * Gizmo
    * Text Input
    * Button Group
* Cursors: Support for `cursor` style property.
* Change QuillPlugin to add bevy_mod_picking plugins if needed:

    .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))

## CSS Vars: Need a way to evaluate efficiently.

Need to cache vars before creating computed.

* CSS Var types:
    * Color
    * Asset Path
    * Length
    * f32 / Scalar

## Alternate approach: React Contexts

The problem with this is that it requires non-constant styles.
