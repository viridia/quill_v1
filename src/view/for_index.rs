use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

pub struct IndexedListItem<V: View + 'static> {
    view: Option<V>,
    state: V::State,
}

impl<V: View + 'static> IndexedListItem<V> {
    fn nodes(&self, vc: &ViewContext) -> NodeSpan {
        self.view.as_ref().unwrap().nodes(vc, &self.state)
    }

    fn collect(&mut self, vc: &mut ViewContext) -> NodeSpan {
        self.view.as_ref().unwrap().assemble(vc, &mut self.state)
    }
}

#[doc(hidden)]
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

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter().map(|item| item.nodes(vc)).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let next_len = self.items.len();
        let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        let mut state: Vec<IndexedListItem<V>> = Vec::with_capacity(next_len);
        child_spans.resize(next_len, NodeSpan::Empty);

        // Append new items
        for i in 0..next_len {
            let view = (self.each)(&self.items[i], i);
            let st = view.build(vc);
            state.push(IndexedListItem {
                view: Some(view),
                state: st,
            });
        }

        state
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        let next_len = self.items.len();
        let mut prev_len = state.len();
        // let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        // child_spans.resize(next_len, NodeSpan::Empty);

        // Overwrite existing items.
        // TODO: Blind overwriting might be a problem here if, for example, we overwrite
        // a text element with a non-text element. Basically we're not razing the old output
        // (because we don't know if we should) and this could cause leftovers. If only views
        // were comparable!
        let mut i = 0usize;
        while i < next_len && i < prev_len {
            let child_state = &mut state[i];
            child_state.view = Some((self.each)(&self.items[i], i));
            child_state
                .view
                .as_ref()
                .unwrap()
                .update(vc, &mut child_state.state);
            // child_spans[i] = child_state.node.clone();
            i += 1;
        }

        // Append new items
        while i < next_len {
            let view = (self.each)(&self.items[i], i);
            let st = view.build(vc);
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
                view.raze(vc, &mut child_state.state);
            }
            state.pop();
        }
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        let child_spans: Vec<NodeSpan> = state.iter_mut().map(|item| item.collect(vc)).collect();
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        let prev_len = state.len();

        let mut i = 0usize;
        while i < prev_len {
            let child_state = &mut state[i];
            if let Some(ref view) = child_state.view {
                view.raze(vc, &mut child_state.state);
            }
            i += 1;
        }
    }
}
