use std::cmp;

#[derive(Debug, Clone)]
pub struct Entry<L, R>(pub L, pub R);
impl<L: PartialEq, R> PartialEq for Entry<L, R> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<L: Eq, R> Eq for Entry<L, R> {}
impl<L: PartialOrd, R> PartialOrd for Entry<L, R> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<L: Ord, R> Ord for Entry<L, R> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
