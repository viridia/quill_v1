# TODO:

* Finish up focus behavior:
    * Space to click
    * Key event to focus
    * Click to defocus
    * focus-visible.
* Change popup menu to use closure for items.
    * Get rid of ViewParam
    * This is much harder than it sounds.
* Floating doesn't reposition on window size.
* Migrate to Egret/Grackle:
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
