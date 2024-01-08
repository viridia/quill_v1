use crate::StyleHandle;
use impl_trait_for_tuples::*;

/// `StyleTuple` - a variable-length tuple of [`StyleHandle`]s.
pub trait StyleTuple: Sync + Send + Clone {
    /// Return the actual number of [`StyleHandle`]s in this tuple (including nesting).
    fn len(&self) -> usize;

    /// True if the tuple is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert the [`StyleHandle`]s into a vector.
    fn collect(&self, v: &mut Vec<StyleHandle>);

    /// Convert the tuple into a vector of [`StyleHandle`]s.
    fn to_vec(&self) -> Vec<StyleHandle> {
        let mut v: Vec<StyleHandle> = Vec::with_capacity(self.len());
        self.collect(&mut v);
        v
    }
}

/// Empty tuple.
impl StyleTuple for () {
    fn len(&self) -> usize {
        0
    }

    fn collect(&self, _v: &mut Vec<StyleHandle>) {}
}

impl StyleTuple for StyleHandle {
    fn len(&self) -> usize {
        1
    }

    fn collect(&self, v: &mut Vec<StyleHandle>) {
        v.push(self.clone());
    }
}

impl StyleTuple for Option<StyleHandle> {
    fn len(&self) -> usize {
        1
    }

    fn collect(&self, v: &mut Vec<StyleHandle>) {
        if let Some(st) = self {
            v.push(st.clone());
        }
    }
}

#[impl_for_tuples(1, 16)]
impl StyleTuple for Tuple {
    for_tuples!( where #( Tuple: StyleTuple )* );

    fn len(&self) -> usize {
        for_tuples!( #( self.Tuple.len() )+* );
    }

    fn collect(&self, v: &mut Vec<StyleHandle>) {
        for_tuples!( #( self.Tuple.collect(v); )* );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to convert a tuple of styles into a vector of style handles.
    fn styles<S: StyleTuple>(items: S) -> Vec<StyleHandle> {
        items.to_vec()
    }

    #[test]
    fn test_style_tuple_empty() {
        let s = styles(());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_style_tuple_single() {
        let s1 = StyleHandle::build(|ss| ss.border(1));
        let s = styles(s1);
        assert_eq!(s.len(), 1);

        let s1 = StyleHandle::build(|ss| ss.border(1));
        let s = styles((s1,));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_style_tuple_2() {
        let s1 = StyleHandle::build(|ss| ss.border(1));
        let s2 = StyleHandle::build(|ss| ss.border(2));
        let s = styles((s1, s2));
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_style_tuple_nested() {
        let s1 = StyleHandle::build(|ss| ss.border(1));
        let s2 = StyleHandle::build(|ss| ss.border(2));
        let s3 = StyleHandle::build(|ss| ss.border(3));
        let s = styles((s1, (s2, s3)));
        assert_eq!(s.len(), 3);
    }
}
