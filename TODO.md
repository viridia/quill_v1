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

```rust
struct AtomHandle<T> {
    marker: PhantomData<T>
    id: ComponentId,
}

let atom = world.create_atom::<T>();

let value = cx.read_atom(atom);
cx.write_atom(atom, new_value);
cx.init_atom(atom, initial_value);

world.read_atom(atom);
world.write_atom(atom, new_value);
world.init_atom(atom, initial_value);
world.is_atom_changed(atom);
```
