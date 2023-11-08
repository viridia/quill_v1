use crate::{ElementContext, View};

use super::node_span::NodeSpan;

pub struct For;

impl For {
    /// Construct an index for loop for an array of items. The callback is called once for each
    /// array element; its arguments are the item and the array index, and its result is a View.
    /// During rebuild, the elements are overwritten based on their current array index, so the
    /// order of child views never changes.
    pub fn index<
        Item: Send + Sync + Clone,
        V: View + 'static,
        F: Fn(&Item, usize) -> V + Sync + Send,
    >(
        items: &[Item],
        each: F,
    ) -> impl View
    where
        V::State: Clone,
    {
        ForIndex::<Item, V, F>::new(items, each)
    }

    //     /// Construct an index for loop for an array of items. The callback is called once for each
    //     /// array element; its argument is the item, which must be equals-comparable, and it's result
    //     /// is a View. During rebuild, the list of child views may be re-ordered based on a comparison
    //     /// of the items from the previous build.
    //     pub fn eq<'a, Item: PartialEq + Sync, V: View>(
    //         items: &'a [Item],
    //         each: fn(item: Item) -> V,
    //     ) -> impl View {
    //         todo!()
    //         // ForEq { items, each }
    //     }

    //     /// Construct an index for loop for an array of items. There are two callbacks, one which
    //     /// produces a unique key for each array item, and one which produces a child view for each
    //     /// array item. During rebuilds, the list of child views may be re-ordered based on a
    //     /// comparison of the generated keys.
    //     pub fn keyed<'a, Item: Sync, Key: PartialEq, V: View>(
    //         items: &'a [Item],
    //         keyof: fn(item: Item) -> Key,
    //         each: fn(item: Item) -> V,
    //     ) -> impl View {
    //         todo!()
    //         // ForKeyed { items, keyof, each }
    //     }
}

pub struct ListItem<Key: Sync + Send + PartialEq, V: View + 'static> {
    view: Option<V>,
    state: V::State,
    key: Key,
    node: NodeSpan,
}

// ForIndex

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
    type State = Vec<ListItem<usize, V>>;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        _prev: &NodeSpan,
    ) -> NodeSpan {
        let count_spans = self.items.len();
        let prev_len = state.len();
        let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(count_spans);
        child_spans.resize(count_spans, NodeSpan::Empty);

        // Overwrite existing items.
        // TODO: We also need to be able to detect if the closure variables have changed
        // in a way that would cause nodes to re-render, even if the array element is the same.
        let mut i = 0usize;
        while i < count_spans && i < prev_len {
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
        while i < count_spans {
            state.push(ListItem {
                view: Some((self.each)(&self.items[i], i)),
                state: Default::default(),
                key: prev_len,
                node: NodeSpan::Empty,
            });
            i += 1;
        }

        // Raze surplus items.
        while prev_len > count_spans {
            todo!()
            // prev_len -= 1;
            // let st = state.pop().as_ref().unwrap();
            // (self.each)(&self.items[prev_len], prev_len).raze(
            //     ecx,
            //     &mut st.state,
            //     &child_spans[prev_len],
            // );
        }

        // TODO: Raze items from previous

        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn raze(&self, _ecx: &mut ElementContext, _state: &mut Self::State, _prev: &NodeSpan) {
        todo!()
    }
}

// ForEq

// pub struct ForEq<'a, Item: PartialEq, V: View> {
//     items: &'a [Item],
//     each: fn(item: Item) -> V,
// }

// impl<'a, Item: PartialEq + Sync, V: View> View for ForEq<'a, Item, V> {
//     type State = Vec<V::State>;

//     fn build(
//         &self,
//         ecx: &mut ElementContext,
//         state: &mut Self::State,
//         prev: &NodeSpan,
//     ) -> NodeSpan {
//         todo!()
//     }

//     fn raze(&self, _ecx: &mut ElementContext, _state: &mut Self::State, prev: &NodeSpan) {
//         todo!()
//     }
// }

// ForKeyed

// pub struct ForKeyed<'a, Item, Key: PartialEq, V: View> {
//     items: &'a [Item],
//     keyof: fn(item: Item) -> Key,
//     each: fn(item: Item) -> V,
// }

// impl<'a, Item: Sync, Key: PartialEq, V: View> View for ForKeyed<'a, Item, Key, V> {
//     type State = Vec<V::State>;

//     fn build(
//         &self,
//         ecx: &mut ElementContext,
//         state: &mut Self::State,
//         prev: &NodeSpan,
//     ) -> NodeSpan {
//         todo!()
//     }

//     fn raze(&self, _ecx: &mut ElementContext, _state: &mut Self::State, prev: &NodeSpan) {
//         todo!()
//     }
//     // each: fn(index: usize, item: Item) -> V,
// }
