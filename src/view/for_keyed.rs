use std::{marker::PhantomData, ops::Range};

use bevy::ecs::world::World;

use crate::{view::lcs::lcs, BuildContext, View};

use crate::node_span::NodeSpan;

pub struct KeyedListItem<Key: Send + PartialEq, V: View> {
    view: Option<V>,
    state: Option<V::State>,
    key: Key,
}

impl<Key: Send + PartialEq, V: View> KeyedListItem<Key, V> {
    fn nodes(&self, bc: &BuildContext) -> NodeSpan {
        self.view
            .as_ref()
            .unwrap()
            .nodes(bc, self.state.as_ref().unwrap())
    }

    fn assemble(&mut self, bc: &mut BuildContext) -> NodeSpan {
        self.view
            .as_ref()
            .unwrap()
            .assemble(bc, self.state.as_mut().unwrap())
    }
}

#[doc(hidden)]
#[allow(clippy::needless_range_loop)]
pub struct ForKeyed<
    Item: Send + Clone,
    Key: Send + PartialEq,
    V: View,
    K: Fn(&Item) -> Key + Send,
    F: Fn(&Item) -> V + Send,
> where
    V::State: Clone,
{
    items: Vec<Item>,
    keyof: K,
    each: F,
    key: PhantomData<Key>,
}

#[allow(clippy::needless_range_loop)]
impl<
        Item: Send + Clone,
        Key: Send + PartialEq,
        V: View,
        K: Fn(&Item) -> Key + Send + Clone,
        F: Fn(&Item) -> V + Send + Clone,
    > ForKeyed<Item, Key, V, K, F>
where
    V::State: Clone,
{
    pub fn new(items: &[Item], keyof: K, each: F) -> Self {
        Self {
            items: Vec::from(items),
            each,
            keyof,
            key: PhantomData::<Key> {},
        }
    }

    /// Uses the sequence of key values to match the previous array items with the updated
    /// array items. Matching items are patched, other items are inserted or deleted.
    ///
    /// # Arguments
    ///
    /// * `bc` - [`BuildContext`] used to build individual elements.
    /// * `prev_state` - Array of view state elements from previous update.
    /// * `prev_range` - The range of elements we are comparing in `prev_state`.
    /// * `next_state` - Array of view state elements to be built.
    /// * `next_range` - The range of elements we are comparing in `next_state`.
    fn build_recursive(
        &self,
        bc: &mut BuildContext,
        prev_state: &mut [KeyedListItem<Key, V>],
        prev_range: Range<usize>,
        next_state: &mut [KeyedListItem<Key, V>],
        next_range: Range<usize>,
    ) {
        // Look for longest common subsequence.
        // prev_start and next_start are *relative to the slice*.
        let (prev_start, next_start, lcs_length) = lcs(
            &prev_state[prev_range.clone()],
            &next_state[next_range.clone()],
            |a, b| a.key == b.key,
        );

        // If there was nothing in common
        if lcs_length == 0 {
            // Raze old elements
            for i in prev_range {
                let prev = &mut prev_state[i];
                if let Some(ref view) = prev.view {
                    view.raze(bc.world, prev.state.as_mut().unwrap());
                }
            }
            // Build new elements
            for i in next_range {
                let next = &mut next_state[i];
                let view = (self.each)(&self.items[i]);
                next.state = Some(view.build(bc));
                next.view = Some(view);
            }
            return;
        }

        // Adjust prev_start and next_start to be relative to the entire state array.
        let prev_start = prev_start + prev_range.start;
        let next_start = next_start + next_range.start;

        // Stuff that precedes the LCS.
        if prev_start > prev_range.start {
            if next_start > next_range.start {
                // Both prev and next have entries before lcs, so recurse
                self.build_recursive(
                    bc,
                    prev_state,
                    prev_range.start..prev_start,
                    next_state,
                    next_range.start..next_start,
                )
            } else {
                // Deletions
                for i in prev_range.start..prev_start {
                    let prev = &mut prev_state[i];
                    if let Some(ref view) = prev.view {
                        view.raze(bc.world, prev.state.as_mut().unwrap());
                    }
                }
            }
        } else if next_start > next_range.start {
            // Insertions
            for i in next_range.start..next_start {
                let next = &mut next_state[i];
                let view = (self.each)(&self.items[i]);
                next.state = Some(view.build(bc));
                next.view = Some(view);
            }
        }

        // For items that match, overwrite.
        for i in 0..lcs_length {
            let prev = &mut prev_state[prev_start + i];
            let next = &mut next_state[next_start + i];
            // Take the old state, update with new View for this element.
            next.state = prev.state.take();
            let v = (self.each)(&self.items[next_start + i]);
            v.update(bc, next.state.as_mut().unwrap());
            next.view = Some(v);
        }

        // Stuff that follows the LCS.
        let prev_end = prev_start + lcs_length;
        let next_end = next_start + lcs_length;
        if prev_end < prev_range.end {
            if next_end < next_range.end {
                // Both prev and next have entries after lcs, so recurse
                self.build_recursive(
                    bc,
                    prev_state,
                    prev_end..prev_range.end,
                    next_state,
                    next_end..next_range.end,
                )
            } else {
                // Deletions
                for i in prev_end..prev_range.end {
                    let prev = &mut prev_state[i];
                    if let Some(ref view) = prev.view {
                        view.raze(bc.world, prev.state.as_mut().unwrap());
                    }
                }
            }
        } else if next_end < next_range.end {
            // Insertions
            for i in next_end..next_range.end {
                let next = &mut next_state[i];
                let view = (self.each)(&self.items[i]);
                next.state = Some(view.build(bc));
                next.view = Some(view);
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
impl<
        Item: Send + Clone,
        Key: Send + PartialEq,
        V: View,
        K: Fn(&Item) -> Key + Send + Clone,
        F: Fn(&Item) -> V + Send + Clone,
    > View for ForKeyed<Item, Key, V, K, F>
where
    V::State: Clone,
{
    type State = Vec<KeyedListItem<Key, V>>;

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter().map(|item| item.nodes(bc)).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        let next_len = self.items.len();
        let mut next_state: Self::State = Vec::with_capacity(next_len);

        // Initialize next state array to default values; fill in keys.
        for j in 0..next_len {
            let view = (self.each)(&self.items[j]);
            let state = view.build(bc);
            next_state.push({
                KeyedListItem {
                    view: Some(view),
                    state: Some(state),
                    key: (self.keyof)(&self.items[j]),
                }
            });
        }

        next_state
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        let next_len = self.items.len();
        let mut next_state: Self::State = Vec::with_capacity(next_len);
        let prev_len = state.len();

        // Initialize output state array; fill in keys.
        for j in 0..next_len {
            next_state.push({
                KeyedListItem {
                    view: None,
                    state: None,
                    key: (self.keyof)(&self.items[j]),
                }
            });
        }

        self.build_recursive(bc, state, 0..prev_len, &mut next_state, 0..next_len);
        for j in 0..next_len {
            assert!(next_state[j].state.is_some(), "Empty state: {}", j);
        }
        std::mem::swap(state, &mut next_state);
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter_mut().map(|item| item.assemble(bc)).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        for child_state in state {
            if let Some(ref view) = child_state.view {
                view.raze(world, child_state.state.as_mut().unwrap());
            }
        }
    }
}

impl<
        Item: Send + Clone,
        Key: Send + PartialEq,
        V: View,
        K: Fn(&Item) -> Key + Send + Clone,
        F: Fn(&Item) -> V + Send + Clone,
    > Clone for ForKeyed<Item, Key, V, K, F>
where
    V::State: Clone,
{
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            keyof: self.keyof.clone(),
            each: self.each.clone(),
            key: self.key,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::world::World;

    use super::*;

    #[test]
    fn test_update() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut bc = BuildContext {
            world: &mut world,
            entity,
        };

        // Initial render
        let view = ForKeyed::new(&[1, 2, 3], |item| *item, |item| format!("{}", item));
        let mut state = view.build(&mut bc);
        assert_eq!(state.len(), 3);
        assert_eq!(state[0].key, 1);
        assert!(state[0].state.is_some());
        assert_eq!(state[1].key, 2);
        assert!(state[1].state.is_some());
        assert_eq!(state[2].key, 3);
        assert!(state[2].state.is_some());
        let e1 = state[0].state;

        // Insert at start
        let view = ForKeyed::new(&[0, 1, 2, 3], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 4);
        assert_eq!(state[0].key, 0);
        assert_eq!(state[3].key, 3);
        assert_eq!(state[1].state, e1, "Should be same entity");

        // Delete at start
        let view = ForKeyed::new(&[1, 2, 3], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 3);
        assert_eq!(state[0].key, 1);
        assert_eq!(state[2].key, 3);
        assert_eq!(state[0].state, e1, "Should be same entity");

        // Insert at end
        let view = ForKeyed::new(&[1, 2, 3, 4], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 4);
        assert_eq!(state[0].key, 1);
        assert_eq!(state[3].key, 4);
        assert_eq!(state[0].state, e1, "Should be same entity");

        // Delete at end
        let view = ForKeyed::new(&[1, 2, 3], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 3);
        assert_eq!(state[0].key, 1);
        assert_eq!(state[2].key, 3);
        assert_eq!(state[0].state, e1, "Should be same entity");

        // Delete in middle
        let view = ForKeyed::new(&[1, 3], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 2);
        assert_eq!(state[0].key, 1);
        assert_eq!(state[1].key, 3);
        assert_eq!(state[0].state, e1, "Should be same entity");

        // Insert in middle
        let view = ForKeyed::new(&[1, 2, 3], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 3);
        assert_eq!(state[0].key, 1);
        assert_eq!(state[1].key, 2);
        assert_eq!(state[2].key, 3);
        assert_eq!(state[0].state, e1, "Should be same entity");

        // Replace in the middle
        let view = ForKeyed::new(&[1, 5, 3], |item| *item, |item| format!("{}", item));
        view.update(&mut bc, &mut state);
        assert_eq!(state.len(), 3);
        assert_eq!(state[0].key, 1);
        assert_eq!(state[1].key, 5);
        assert_eq!(state[2].key, 3);
        assert_eq!(state[0].state, e1, "Should be same entity");
    }
}
