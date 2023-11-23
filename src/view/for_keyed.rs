use std::{marker::PhantomData, ops::Range};

use crate::{view::lcs::lcs, ElementContext, View};

use crate::node_span::NodeSpan;

pub struct KeyedListItem<Key: Sync + Send + PartialEq, V: View + 'static> {
    view: Option<V>,
    state: Option<V::State>,
    key: Key,
    node: NodeSpan,
}

pub struct ForKeyed<
    Item: Sync + Send + Clone,
    Key: Sync + Send + PartialEq,
    V: View + 'static,
    K: Fn(&Item) -> Key + Sync + Send,
    F: Fn(&Item) -> V + Sync + Send,
> where
    V::State: Clone,
{
    items: Vec<Item>,
    keyof: K,
    each: F,
    key: PhantomData<Key>,
}

impl<
        Item: Sync + Send + Clone,
        Key: Sync + Send + PartialEq,
        V: View + 'static,
        K: Fn(&Item) -> Key + Sync + Send,
        F: Fn(&Item) -> V + Sync + Send,
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
    fn build_recursive(
        &self,
        ecx: &mut ElementContext,
        prev_state: &mut [KeyedListItem<Key, V>],
        prev_range: Range<usize>,
        next_state: &mut [KeyedListItem<Key, V>],
        next_range: Range<usize>,
    ) {
        // Look for longest common subsequence
        let (prev_start, next_start, length) = lcs(
            &prev_state[prev_range.clone()],
            &next_state[next_range.clone()],
            |a, b| a.key == b.key,
        );

        // Stuff that precedes the LCS.
        if prev_start > prev_range.start {
            if next_start > next_range.start {
                // Both prev and next have entries before lcs, so recurse
                self.build_recursive(
                    ecx,
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
                        view.raze(ecx, prev.state.as_mut().unwrap(), &prev.node);
                    }
                }
            }
        } else if next_start > next_range.start {
            // Insertions
            for i in next_range.start..next_start {
                let next = &mut next_state[i];
                let view = (self.each)(&self.items[i]);
                let (st, node) = view.build(ecx);
                next.view = Some(view);
                next.state = Some(st);
                next.node = node;
            }
        }

        // For items that match, overwrite.
        for i in 0..length {
            let prev = &mut prev_state[prev_start + i];
            let next = &mut next_state[next_start + i];
            next.view = Some((self.each)(&self.items[i]));
            next.node =
                prev.view
                    .as_ref()
                    .unwrap()
                    .rebuild(ecx, prev.state.as_mut().unwrap(), &prev.node);
        }

        // Stuff that follows the LCS.
        let prev_end = prev_start + length;
        let next_end = next_start + length;
        if prev_end < prev_range.end {
            if next_end < next_range.end {
                // Both prev and next have entries after lcs, so recurse
                self.build_recursive(
                    ecx,
                    prev_state,
                    prev_end..prev_range.end,
                    next_state,
                    next_end..next_range.end,
                )
            } else {
                // Deletions
                for i in next_end..next_range.end {
                    let prev = &mut prev_state[i];
                    if let Some(ref view) = prev.view {
                        view.raze(ecx, prev.state.as_mut().unwrap(), &prev.node);
                    }
                }
            }
        } else if next_end < next_range.end {
            // Insertions
            for i in next_end..next_range.end {
                let next = &mut next_state[i];
                next.view = Some((self.each)(&self.items[i]));
                let view = (self.each)(&self.items[i]);
                let (st, node) = view.build(ecx);
                next.view = Some(view);
                next.state = Some(st);
                next.node = node;
            }
        }
    }
}

impl<
        Item: Sync + Send + Clone,
        Key: Sync + Send + PartialEq,
        V: View + 'static,
        K: Fn(&Item) -> Key + Sync + Send,
        F: Fn(&Item) -> V + Sync + Send,
    > View for ForKeyed<Item, Key, V, K, F>
where
    V::State: Clone,
{
    type State = Vec<KeyedListItem<Key, V>>;

    fn build(&self, ecx: &mut ElementContext) -> (Self::State, NodeSpan) {
        let next_len = self.items.len();
        let mut next_state: Self::State = Vec::with_capacity(next_len);
        let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);

        // Initialize next state array to default values; fill in keys.
        for j in 0..next_len {
            let view = (self.each)(&self.items[j]);
            let (state, node) = view.build(ecx);
            child_spans.push(node.clone());
            next_state.push({
                KeyedListItem {
                    view: Some(view),
                    state: Some(state),
                    key: (self.keyof)(&self.items[j]),
                    node,
                }
            });
        }

        (
            next_state,
            NodeSpan::Fragment(child_spans.into_boxed_slice()),
        )
    }

    fn rebuild(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        _prev: &NodeSpan,
    ) -> NodeSpan {
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
                    node: NodeSpan::Empty,
                }
            });
        }

        self.build_recursive(ecx, state, 0..prev_len, &mut next_state, 0..next_len);
        std::mem::swap(state, &mut next_state);

        let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        for st in state {
            child_spans.push(st.node.clone())
        }
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn collect(
        &self,
        _ecx: &mut ElementContext,
        state: &mut Self::State,
        nodes: &NodeSpan,
    ) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter().map(|item| item.node.clone()).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, _prev: &NodeSpan) {
        for i in 0..state.len() {
            let child_state = &mut state[i];
            if let Some(ref view) = child_state.view {
                view.raze(ecx, child_state.state.as_mut().unwrap(), &child_state.node);
            }
        }
    }
}
