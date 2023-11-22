# TODO:

* Race conditions in ViewHandles. Yuck.
* Work on cargo docs.
* Some kind of sugar for stylesets that avoids all the Arc::new() and .clone() calls.
* Change style builder methods to accept `impl StyleValue`. Then define `Var("varname")`
* Leaf nodes render when dependencies change (currently render unconditionally).
* Option for ViewHandle to parent to an explicit (manually constructed) UiNode.
* Experiment with dependency injection instead of hooks.
* Detect changes to presenter props.
* `use` hooks for components, events, etc.
* Implement button click handler. (Right now using an event, but closure would be better).
* Basic widgets: Button, Slider, Text Input.
* ElementContext shouldn't need to be mutable except for world - use a refcell?
* Support for `cursor` style property.
* Rename to 'bevy_quill' since 'quill' is taken.
* Change QuillPlugin to add bevy_mod_picking plugins if needed:

    .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))

enum NodeSliceSpan {
  None,
  Entity(Entity),
  Fragment(Arc<NodeSlice>),
}

struct NodeSlice {
  changed: Arc<AtomicInt>;
  content: NodeSliceSpan,

  fn count(&self) -> usize;

  fn flat(&self, &[Entity]);

  fn clear(&mut self) {
    if content != NodeSliceSpan::Empty {
      self.content = NodeSliceSpan::Empty;
      self.changed.set(true)
    }
  }

  fn replace(&mut self, entity: Entity) {
    let content = NodeSliceSpan::Entity(entity);
    if content != self.content {
      self.content = content;
      self.changed.set(true)
    }
  }

  fn slice(&mut self, size: usize) -> &[mut NodeSpan] {

  }

  fn resize(&mut self, size: usize) {
    match self.content {
      NodeSliceSpan::Fragment(ref children)

    }
  }

  fn at(&mut self, index: usize) -> NodeSlice {
    return NodeSlice {
      changed: self.changed.clone(),
      content: self.content.
    }
  }
}

out.replace(entity);
out.resize(3);
out.at(0).replace(entity);
out.at(1).replace(entity);
out.at(2).replace(entity);
