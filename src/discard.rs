/// Types that can be safely discarded, ex. unit [`()`]
pub trait Discard {}

impl Discard for () {}
