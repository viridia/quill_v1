use crate::{ElementContext, View};

use crate::node_span::NodeSpan;

pub struct IndexedListItem<V: View + 'static> {
    view: Option<V>,
    state: V::State,
}

impl<V: View + 'static> IndexedListItem<V> {
    fn nodes(&self, ecx: &ElementContext) -> NodeSpan {
        self.view.as_ref().unwrap().nodes(ecx, &self.state)
    }

    fn collect(&mut self, ecx: &mut ElementContext) -> NodeSpan {
        self.view.as_ref().unwrap().collect(ecx, &mut self.state)
    }
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

    fn nodes(&self, ecx: &ElementContext, state: &Self::State) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter().map(|item| item.nodes(ecx)).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn build(&self, ecx: &mut ElementContext) -> Self::State {
        let next_len = self.items.len();
        let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        let mut state: Vec<IndexedListItem<V>> = Vec::with_capacity(next_len);
        child_spans.resize(next_len, NodeSpan::Empty);

        // Append new items
        for i in 0..next_len {
            let view = (self.each)(&self.items[i], i);
            let st = view.build(ecx);
            state.push(IndexedListItem {
                view: Some(view),
                state: st,
            });
        }

        state
    }

    fn rebuild(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        let next_len = self.items.len();
        let mut prev_len = state.len();
        // let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        // child_spans.resize(next_len, NodeSpan::Empty);

        // Overwrite existing items.
        let mut i = 0usize;
        while i < next_len && i < prev_len {
            let child_state = &mut state[i];
            child_state.view = Some((self.each)(&self.items[i], i));
            child_state
                .view
                .as_ref()
                .unwrap()
                .rebuild(ecx, &mut child_state.state);
            // child_spans[i] = child_state.node.clone();
            i += 1;
        }

        // Append new items
        while i < next_len {
            let view = (self.each)(&self.items[i], i);
            let st = view.build(ecx);
            state.push(IndexedListItem {
                view: Some(view),
                state: st,
            });
            i += 1;
        }

        // Raze surplus items.
        while i < prev_len {
            prev_len -= 1;
            let child_state = &mut state[prev_len];
            if let Some(ref view) = child_state.view {
                view.raze(ecx, &mut child_state.state);
            }
            state.pop();
        }
    }

    fn collect(&self, ecx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter_mut().map(|item| item.collect(ecx)).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        let prev_len = state.len();

        let mut i = 0usize;
        while i < prev_len {
            let child_state = &mut state[i];
            if let Some(ref view) = child_state.view {
                view.raze(ecx, &mut child_state.state);
            }
            i += 1;
        }
    }
}
