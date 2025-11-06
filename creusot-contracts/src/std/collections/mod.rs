use crate::prelude::*;
use std::collections::*;

impl View for TryReserveError {
    type ViewTy = Self;

    #[logic(open)]
    fn view(self) -> Self::ViewTy {
        self
    }
}

impl DeepModel for TryReserveError {
    type DeepModelTy = Self;

    #[logic(open, inline)]
    fn deep_model(self) -> Self::DeepModelTy {
        pearlite! { self@ }
    }
}

pub mod hash_map;
pub mod hash_set;
