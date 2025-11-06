use crate::prelude::*;
use std::alloc::*;

impl View for Layout {
    type ViewTy = (usize, usize);

    #[trusted]
    #[logic(opaque)]
    fn view(self) -> Self::ViewTy {
        dead
    }
}

impl DeepModel for Layout {
    type DeepModelTy = (usize, usize);

    #[logic(open, inline)]
    fn deep_model(self) -> Self::DeepModelTy {
        pearlite! { self@ }
    }
}

extern_spec! {
    impl Layout {
        #[ensures(forall<l: Layout> result == Ok(l) ==> l@.0 == size && l@.1 == align)]
        fn from_size_align(size: usize, align: usize) -> Result<Layout, LayoutError>;

        #[ensures(result == self@.0)]
        fn size(&self) -> usize;

        #[ensures(result == self@.1)]
        fn align(&self) -> usize;
    }
}
