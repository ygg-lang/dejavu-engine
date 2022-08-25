use super::*;

impl<T: PartialEq> PartialEq<Self> for OrderedSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }
        for (lhs, rhs) in self.inner.iter().zip(other.inner.iter()) {
            if lhs.ne(rhs) {
                return false;
            }
        }
        return true;
    }
}

impl<T: Eq> PartialEq<Self> for KVPair<T> {
    fn eq(&self, other: &Self) -> bool {
        let k = self.key.value.eq(&other.key.value);
        let v = self.value.value.eq(&other.value.value);
        k && v
    }
}

impl<T: Eq> Eq for KVPair<T> {}
