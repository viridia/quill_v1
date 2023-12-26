use super::{for_index::ForIndex, for_keyed::ForKeyed, View};

/// A namespace that contains constructor functions for various kinds of for-loops:
/// * `For::each()`
/// * `For::keyed()`
/// * `For::index()`
pub struct For;

impl For {
    /// Construct an index for loop for an array of items. The callback is called once for each
    /// array element; its arguments are the item and the array index, and its result is a View.
    /// During rebuild, the elements are overwritten based on their current array index, so the
    /// order of child views never changes.
    pub fn index<Item: Send + Clone, V: View, F: Fn(&Item, usize) -> V + Send + Clone>(
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
        Item: Send + Clone,
        Key: Send + PartialEq,
        V: View,
        K: Fn(&Item) -> Key + Send + Clone,
        F: Fn(&Item) -> V + Send + Clone,
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
    pub fn each<Item: Send + Clone + PartialEq, V: View, F: Fn(&Item) -> V + Send + Clone>(
        items: &[Item],
        each: F,
    ) -> impl View
    where
        V::State: Clone,
    {
        ForKeyed::new(items, |item| item.clone(), each)
    }
}
