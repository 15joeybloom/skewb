use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone, Copy)]
pub struct UnorderedPair<T> {
    pub one: T,
    pub two: T,
}

impl<T> UnorderedPair<T>
where T: Eq
{
    pub fn new(one: T, two: T) -> UnorderedPair<T> { UnorderedPair::<T> { one, two } }
    pub fn contains(&self, t: &T) -> bool { self.one == *t || self.two == *t }
    pub fn disjoint(&self, other: &UnorderedPair<T>) -> bool {
        !self.contains(&other.one) && !self.contains(&other.two)
    }
}

impl<T> PartialEq for UnorderedPair<T>
where T: PartialEq
{
    fn eq(&self, other: &UnorderedPair<T>) -> bool {
        (self.one == other.one && self.two == other.two)
            || (self.two == other.one && self.one == other.two)
    }
}

impl<T> Eq for UnorderedPair<T> where T: Eq {}

impl<T> Hash for UnorderedPair<T>
where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        fn get_hash<T: Hash>(t: &T) -> u64 {
            let mut state = DefaultHasher::new();
            t.hash(&mut state);
            state.finish()
        }
        let h1 = get_hash(&self.one);
        let h2 = get_hash(&self.two);
        (min(h1, h2)).hash(state);
        (max(h1, h2)).hash(state);
    }
}

#[test]
fn contains() {
    let x = UnorderedPair::new(2, 4);
    assert!(x.contains(&2));
    assert!(x.contains(&4));
    assert!(!x.contains(&3));
}

#[test]
fn disjoint() {
    let x = UnorderedPair::new(2, 4);
    let y = UnorderedPair::new(4, 2);
    let z = UnorderedPair::new(4, 3);
    let w = UnorderedPair::new(5, 3);

    assert!(!x.disjoint(&x));
    assert!(!x.disjoint(&y));
    assert!(!x.disjoint(&z));
    assert!(!z.disjoint(&w));

    assert!(x.disjoint(&w));
}

#[test]
fn eq() {
    let x = UnorderedPair::new(2, 4);
    let y = UnorderedPair::new(4, 2);
    let z = UnorderedPair::new(4, 3);
    assert_eq!(x, x);
    assert_eq!(y, y);
    assert_eq!(z, z);

    assert_eq!(x, y);
    assert_ne!(x, z);
    assert_ne!(y, z);
}

#[test]
fn hash() {
    let x = UnorderedPair::new(2, 4);
    let y = UnorderedPair::new(4, 2);
    let z = UnorderedPair::new(4, 3);

    use std::collections::HashSet;
    let mut set = HashSet::new();
    assert!(set.insert(x));
    assert!(set.contains(&y));

    assert!(!set.insert(y));
    assert!(set.insert(z));
    assert!(!set.insert(y));

    assert!(set.contains(&x));
    assert!(set.contains(&y));
    assert!(set.contains(&z));

    set.remove(&y);
    assert!(!set.contains(&x));
}
