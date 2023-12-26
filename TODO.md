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


trait Memo<T> {
    fn update(&mut self, new_val: T) -> bool {}


}
