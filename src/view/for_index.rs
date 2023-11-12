use crate::{ElementContext, View};

use crate::node_span::NodeSpan;

pub struct IndexedListItem<V: View + 'static> {
    view: Option<V>,
    state: V::State,
    node: NodeSpan,
}

pub struct ForIndex<
    Item: Sync + Send + Clone,
    V: View + 'static,
    F: Fn(&Item, usize) -> V + Sync + Send,
> where
    V::State: Clone,
{
    items: Vec<Item>,
    each: F,
}

impl<Item: Sync + Send + Clone, V: View + 'static, F: Fn(&Item, usize) -> V + Sync + Send>
    ForIndex<Item, V, F>
where
    V::State: Clone,
{
    pub fn new(items: &[Item], each: F) -> Self {
        Self {
            items: Vec::from(items),
            each,
        }
    }
}

impl<Item: Sync + Send + Clone, V: View + 'static, F: Fn(&Item, usize) -> V + Sync + Send> View
    for ForIndex<Item, V, F>
where
    V::State: Clone,
{
    type State = Vec<IndexedListItem<V>>;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        _prev: &NodeSpan,
    ) -> NodeSpan {
        let next_len = self.items.len();
        let mut prev_len = state.len();
        let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        child_spans.resize(next_len, NodeSpan::Empty);

        // Overwrite existing items.
        let mut i = 0usize;
        while i < next_len && i < prev_len {
            let child_state = &mut state[i];
            child_state.view = Some((self.each)(&self.items[i], i));
            child_state.node = child_state.view.as_ref().unwrap().build(
                ecx,
                &mut child_state.state,
                &child_state.node,
            );
            child_spans[i] = child_state.node.clone();
            i += 1;
        }

        // Append new items
        while i < next_len {
            state.push(IndexedListItem {
                view: Some((self.each)(&self.items[i], i)),
                state: Default::default(),
                node: NodeSpan::Empty,
            });
            i += 1;
        }

        // Raze surplus items.
        while i < prev_len {
            prev_len -= 1;
            let child_state = &mut state[prev_len];
            if let Some(ref view) = child_state.view {
                view.raze(ecx, &mut child_state.state, &child_state.node);
            }
            state.pop();
        }

        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, _prev: &NodeSpan) {
        let prev_len = state.len();

        let mut i = 0usize;
        while i < prev_len {
            let child_state = &mut state[i];
            if let Some(ref view) = child_state.view {
                view.raze(ecx, &mut child_state.state, &child_state.node);
            }
            i += 1;
        }
    }
}
