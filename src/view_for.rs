use crate::{view_for_index::ForIndex, view_for_keyed::ForKeyed, View};

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

    /// Construct an keyed for loop for an array of items. There are two callbacks, one which
    /// produces a unique key for each array item, and one which produces a child view for each
    /// array item. During rebuilds, the list of child views may be re-ordered based on a
    /// comparison of the generated keys.
    pub fn keyed<
        Item: Send + Sync + Clone,
        Key: Sync + Send + PartialEq,
        V: View + 'static,
        K: Fn(&Item) -> Key + Sync + Send,
        F: Fn(&Item) -> V + Sync + Send,
    >(
        items: &[Item],
        keyof: K,
        each: F,
    ) -> impl View
    where
        V::State: Clone,
    {
        ForKeyed::new(items, keyof, each)
    }

    /// Construct an unkeyed for loop for an array of items. The callback is called once for each
    /// array element; its argument is the item, which must be equals-comparable, and it's result
    /// is a View. During rebuild, the list of child views may be re-ordered based on a comparison
    /// of the items from the previous build.
    pub fn each<
        Item: Send + Sync + Clone + PartialEq,
        V: View + 'static,
        F: Fn(&Item) -> V + Sync + Send,
    >(
        items: &[Item],
        each: F,
    ) -> impl View
    where
        V::State: Clone,
    {
        ForKeyed::new(items, |item| item.clone(), each)
    }
}
