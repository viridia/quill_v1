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

## Notes on scrolling:

* Scrolling needs to be a Component I think.
* Scrolling implies overflow: hidden, position: absolute, and controls left/top.
* Scrolling automatically creates a scrollbar.

struct Scrolling {
    enable_x: bool,
    enable_y: bool,
    scroll_left: f32,
    scroll_top: f32,
    scroll_width: f32,
    scroll_height: f32,
}

scroll_system:
    measure size and update scroll width and height
    clamp scroll position
    adjust transform
    adjust scrollbar
