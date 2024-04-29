use core::{
    cmp::Ordering,
    iter::{Product, Sum},
};
pub mod iter1 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter1<I0> {
        I0(I0),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0(self) -> Iter1<Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0(self) -> Iter1<Self::IntoIter> {
            Iter1::I0(self.into_iter())
        }
    }
    impl<Item, I0> Iterator for Iter1<I0>
    where
        I0: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0> DoubleEndedIterator for Iter1<I0>
    where
        I0: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0> ExactSizeIterator for Iter1<I0>
    where
        I0: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
            }
        }
    }
    impl<Item, I0> FusedIterator for Iter1<I0> where I0: FusedIterator<Item = Item> {}
}
pub mod iter2 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter2<I0, I1> {
        I0(I0),
        I1(I1),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1>(self) -> Iter2<Self::IntoIter, I1>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1>(self) -> Iter2<Self::IntoIter, I1> {
            Iter2::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0>(self) -> Iter2<I0, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0>(self) -> Iter2<I0, Self::IntoIter> {
            Iter2::I1(self.into_iter())
        }
    }
    impl<Item, I0, I1> Iterator for Iter2<I0, I1>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1> DoubleEndedIterator for Iter2<I0, I1>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1> ExactSizeIterator for Iter2<I0, I1>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1> FusedIterator for Iter2<I0, I1>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter3 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter3<I0, I1, I2> {
        I0(I0),
        I1(I1),
        I2(I2),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2>(self) -> Iter3<Self::IntoIter, I1, I2>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2>(self) -> Iter3<Self::IntoIter, I1, I2> {
            Iter3::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2>(self) -> Iter3<I0, Self::IntoIter, I2>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2>(self) -> Iter3<I0, Self::IntoIter, I2> {
            Iter3::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1>(self) -> Iter3<I0, I1, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1>(self) -> Iter3<I0, I1, Self::IntoIter> {
            Iter3::I2(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2> Iterator for Iter3<I0, I1, I2>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2> DoubleEndedIterator for Iter3<I0, I1, I2>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2> ExactSizeIterator for Iter3<I0, I1, I2>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2> FusedIterator for Iter3<I0, I1, I2>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter4 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter4<I0, I1, I2, I3> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3>(self) -> Iter4<Self::IntoIter, I1, I2, I3>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3>(self) -> Iter4<Self::IntoIter, I1, I2, I3> {
            Iter4::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3>(self) -> Iter4<I0, Self::IntoIter, I2, I3>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3>(self) -> Iter4<I0, Self::IntoIter, I2, I3> {
            Iter4::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3>(self) -> Iter4<I0, I1, Self::IntoIter, I3>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3>(self) -> Iter4<I0, I1, Self::IntoIter, I3> {
            Iter4::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2>(self) -> Iter4<I0, I1, I2, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2>(self) -> Iter4<I0, I1, I2, Self::IntoIter> {
            Iter4::I3(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3> Iterator for Iter4<I0, I1, I2, I3>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3> DoubleEndedIterator for Iter4<I0, I1, I2, I3>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3> ExactSizeIterator for Iter4<I0, I1, I2, I3>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3> FusedIterator for Iter4<I0, I1, I2, I3>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter5 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter5<I0, I1, I2, I3, I4> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4>(self) -> Iter5<Self::IntoIter, I1, I2, I3, I4>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4>(self) -> Iter5<Self::IntoIter, I1, I2, I3, I4> {
            Iter5::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4>(self) -> Iter5<I0, Self::IntoIter, I2, I3, I4>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4>(self) -> Iter5<I0, Self::IntoIter, I2, I3, I4> {
            Iter5::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4>(self) -> Iter5<I0, I1, Self::IntoIter, I3, I4>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4>(self) -> Iter5<I0, I1, Self::IntoIter, I3, I4> {
            Iter5::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4>(self) -> Iter5<I0, I1, I2, Self::IntoIter, I4>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4>(self) -> Iter5<I0, I1, I2, Self::IntoIter, I4> {
            Iter5::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3>(self) -> Iter5<I0, I1, I2, I3, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3>(self) -> Iter5<I0, I1, I2, I3, Self::IntoIter> {
            Iter5::I4(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4> Iterator for Iter5<I0, I1, I2, I3, I4>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4> DoubleEndedIterator for Iter5<I0, I1, I2, I3, I4>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4> ExactSizeIterator for Iter5<I0, I1, I2, I3, I4>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4> FusedIterator for Iter5<I0, I1, I2, I3, I4>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter6 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter6<I0, I1, I2, I3, I4, I5> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5>(self) -> Iter6<Self::IntoIter, I1, I2, I3, I4, I5>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5>(self) -> Iter6<Self::IntoIter, I1, I2, I3, I4, I5> {
            Iter6::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5>(self) -> Iter6<I0, Self::IntoIter, I2, I3, I4, I5>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5>(self) -> Iter6<I0, Self::IntoIter, I2, I3, I4, I5> {
            Iter6::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5>(self) -> Iter6<I0, I1, Self::IntoIter, I3, I4, I5>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5>(self) -> Iter6<I0, I1, Self::IntoIter, I3, I4, I5> {
            Iter6::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5>(self) -> Iter6<I0, I1, I2, Self::IntoIter, I4, I5>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5>(self) -> Iter6<I0, I1, I2, Self::IntoIter, I4, I5> {
            Iter6::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5>(self) -> Iter6<I0, I1, I2, I3, Self::IntoIter, I5>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5>(self) -> Iter6<I0, I1, I2, I3, Self::IntoIter, I5> {
            Iter6::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4>(self) -> Iter6<I0, I1, I2, I3, I4, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4>(self) -> Iter6<I0, I1, I2, I3, I4, Self::IntoIter> {
            Iter6::I5(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5> Iterator for Iter6<I0, I1, I2, I3, I4, I5>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5> DoubleEndedIterator for Iter6<I0, I1, I2, I3, I4, I5>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5> ExactSizeIterator for Iter6<I0, I1, I2, I3, I4, I5>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5> FusedIterator for Iter6<I0, I1, I2, I3, I4, I5>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter7 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter7<I0, I1, I2, I3, I4, I5, I6> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
        I6(I6),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6>(
            self,
        ) -> Iter7<Self::IntoIter, I1, I2, I3, I4, I5, I6>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6>(
            self,
        ) -> Iter7<Self::IntoIter, I1, I2, I3, I4, I5, I6> {
            Iter7::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6>(
            self,
        ) -> Iter7<I0, Self::IntoIter, I2, I3, I4, I5, I6>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6>(
            self,
        ) -> Iter7<I0, Self::IntoIter, I2, I3, I4, I5, I6> {
            Iter7::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6>(
            self,
        ) -> Iter7<I0, I1, Self::IntoIter, I3, I4, I5, I6>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6>(
            self,
        ) -> Iter7<I0, I1, Self::IntoIter, I3, I4, I5, I6> {
            Iter7::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6>(
            self,
        ) -> Iter7<I0, I1, I2, Self::IntoIter, I4, I5, I6>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6>(
            self,
        ) -> Iter7<I0, I1, I2, Self::IntoIter, I4, I5, I6> {
            Iter7::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6>(
            self,
        ) -> Iter7<I0, I1, I2, I3, Self::IntoIter, I5, I6>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6>(
            self,
        ) -> Iter7<I0, I1, I2, I3, Self::IntoIter, I5, I6> {
            Iter7::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6>(
            self,
        ) -> Iter7<I0, I1, I2, I3, I4, Self::IntoIter, I6>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6>(
            self,
        ) -> Iter7<I0, I1, I2, I3, I4, Self::IntoIter, I6> {
            Iter7::I5(self.into_iter())
        }
    }
    pub trait IntoIter6: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5>(
            self,
        ) -> Iter7<I0, I1, I2, I3, I4, I5, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter6 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5>(
            self,
        ) -> Iter7<I0, I1, I2, I3, I4, I5, Self::IntoIter> {
            Iter7::I6(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6> Iterator for Iter7<I0, I1, I2, I3, I4, I5, I6>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
        I6: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
                Self::I6(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
                Self::I6(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
                Self::I6(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
                Self::I6(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
                Self::I6(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
                Self::I6(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
                Self::I6(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
                Self::I6(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
                Self::I6(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
                Self::I6(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
                Self::I6(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
                Self::I6(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
                Self::I6(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
                Self::I6(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
                Self::I6(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
                Self::I6(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
                Self::I6(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
                Self::I6(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
                Self::I6(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
                Self::I6(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
                Self::I6(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
                Self::I6(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
                Self::I6(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
                Self::I6(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
                Self::I6(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
                Self::I6(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
                Self::I6(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
                Self::I6(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
                Self::I6(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
                Self::I6(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
                Self::I6(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6> DoubleEndedIterator for Iter7<I0, I1, I2, I3, I4, I5, I6>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
        I6: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
                Self::I6(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
                Self::I6(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
                Self::I6(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
                Self::I6(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6> ExactSizeIterator for Iter7<I0, I1, I2, I3, I4, I5, I6>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
        I6: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
                Self::I6(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6> FusedIterator for Iter7<I0, I1, I2, I3, I4, I5, I6>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
        I6: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter8 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter8<I0, I1, I2, I3, I4, I5, I6, I7> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
        I6(I6),
        I7(I7),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter8<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter8<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7> {
            Iter8::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter8<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter8<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7> {
            Iter8::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter8<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter8<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7> {
            Iter8::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7>(
            self,
        ) -> Iter8<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7>(
            self,
        ) -> Iter8<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7> {
            Iter8::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7>(
            self,
        ) -> Iter8<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7>(
            self,
        ) -> Iter8<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7> {
            Iter8::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7>(
            self,
        ) -> Iter8<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7>(
            self,
        ) -> Iter8<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7> {
            Iter8::I5(self.into_iter())
        }
    }
    pub trait IntoIter6: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7>(
            self,
        ) -> Iter8<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7>;
    }
    impl<T: IntoIterator> IntoIter6 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7>(
            self,
        ) -> Iter8<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7> {
            Iter8::I6(self.into_iter())
        }
    }
    pub trait IntoIter7: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6>(
            self,
        ) -> Iter8<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter7 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6>(
            self,
        ) -> Iter8<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter> {
            Iter8::I7(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7> Iterator for Iter8<I0, I1, I2, I3, I4, I5, I6, I7>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
        I6: Iterator<Item = Item>,
        I7: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
                Self::I6(this) => this.next(),
                Self::I7(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
                Self::I6(this) => this.size_hint(),
                Self::I7(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
                Self::I6(this) => this.count(),
                Self::I7(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
                Self::I6(this) => this.last(),
                Self::I7(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
                Self::I6(this) => this.nth(n),
                Self::I7(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
                Self::I6(this) => this.for_each(f),
                Self::I7(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
                Self::I6(this) => this.collect(),
                Self::I7(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
                Self::I6(this) => this.partition(f),
                Self::I7(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
                Self::I6(this) => this.fold(init, f),
                Self::I7(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
                Self::I6(this) => this.reduce(f),
                Self::I7(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
                Self::I6(this) => this.all(f),
                Self::I7(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
                Self::I6(this) => this.any(f),
                Self::I7(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
                Self::I6(this) => this.find(predicate),
                Self::I7(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
                Self::I6(this) => this.find_map(f),
                Self::I7(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
                Self::I6(this) => this.position(predicate),
                Self::I7(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
                Self::I6(this) => this.max(),
                Self::I7(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
                Self::I6(this) => this.min(),
                Self::I7(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
                Self::I6(this) => this.max_by_key(f),
                Self::I7(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
                Self::I6(this) => this.max_by(compare),
                Self::I7(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
                Self::I6(this) => this.min_by_key(f),
                Self::I7(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
                Self::I6(this) => this.min_by(compare),
                Self::I7(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
                Self::I6(this) => this.sum(),
                Self::I7(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
                Self::I6(this) => this.product(),
                Self::I7(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
                Self::I6(this) => this.cmp(other),
                Self::I7(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
                Self::I6(this) => this.partial_cmp(other),
                Self::I7(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
                Self::I6(this) => this.eq(other),
                Self::I7(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
                Self::I6(this) => this.ne(other),
                Self::I7(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
                Self::I6(this) => this.lt(other),
                Self::I7(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
                Self::I6(this) => this.le(other),
                Self::I7(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
                Self::I6(this) => this.gt(other),
                Self::I7(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
                Self::I6(this) => this.ge(other),
                Self::I7(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7> DoubleEndedIterator
        for Iter8<I0, I1, I2, I3, I4, I5, I6, I7>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
        I6: DoubleEndedIterator<Item = Item>,
        I7: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
                Self::I6(this) => this.next_back(),
                Self::I7(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
                Self::I6(this) => this.nth_back(n),
                Self::I7(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
                Self::I6(this) => this.rfind(predicate),
                Self::I7(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
                Self::I6(this) => this.rfold(init, f),
                Self::I7(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7> ExactSizeIterator
        for Iter8<I0, I1, I2, I3, I4, I5, I6, I7>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
        I6: ExactSizeIterator<Item = Item>,
        I7: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
                Self::I6(this) => this.len(),
                Self::I7(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7> FusedIterator for Iter8<I0, I1, I2, I3, I4, I5, I6, I7>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
        I6: FusedIterator<Item = Item>,
        I7: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter9 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
        I6(I6),
        I7(I7),
        I8(I8),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8> {
            Iter9::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8> {
            Iter9::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8> {
            Iter9::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8> {
            Iter9::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8> {
            Iter9::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8> {
            Iter9::I5(self.into_iter())
        }
    }
    pub trait IntoIter6: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8>;
    }
    impl<T: IntoIterator> IntoIter6 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8> {
            Iter9::I6(self.into_iter())
        }
    }
    pub trait IntoIter7: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8>;
    }
    impl<T: IntoIterator> IntoIter7 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8> {
            Iter9::I7(self.into_iter())
        }
    }
    pub trait IntoIter8: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter8 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7>(
            self,
        ) -> Iter9<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter> {
            Iter9::I8(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8> Iterator
        for Iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
        I6: Iterator<Item = Item>,
        I7: Iterator<Item = Item>,
        I8: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
                Self::I6(this) => this.next(),
                Self::I7(this) => this.next(),
                Self::I8(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
                Self::I6(this) => this.size_hint(),
                Self::I7(this) => this.size_hint(),
                Self::I8(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
                Self::I6(this) => this.count(),
                Self::I7(this) => this.count(),
                Self::I8(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
                Self::I6(this) => this.last(),
                Self::I7(this) => this.last(),
                Self::I8(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
                Self::I6(this) => this.nth(n),
                Self::I7(this) => this.nth(n),
                Self::I8(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
                Self::I6(this) => this.for_each(f),
                Self::I7(this) => this.for_each(f),
                Self::I8(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
                Self::I6(this) => this.collect(),
                Self::I7(this) => this.collect(),
                Self::I8(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
                Self::I6(this) => this.partition(f),
                Self::I7(this) => this.partition(f),
                Self::I8(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
                Self::I6(this) => this.fold(init, f),
                Self::I7(this) => this.fold(init, f),
                Self::I8(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
                Self::I6(this) => this.reduce(f),
                Self::I7(this) => this.reduce(f),
                Self::I8(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
                Self::I6(this) => this.all(f),
                Self::I7(this) => this.all(f),
                Self::I8(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
                Self::I6(this) => this.any(f),
                Self::I7(this) => this.any(f),
                Self::I8(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
                Self::I6(this) => this.find(predicate),
                Self::I7(this) => this.find(predicate),
                Self::I8(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
                Self::I6(this) => this.find_map(f),
                Self::I7(this) => this.find_map(f),
                Self::I8(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
                Self::I6(this) => this.position(predicate),
                Self::I7(this) => this.position(predicate),
                Self::I8(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
                Self::I6(this) => this.max(),
                Self::I7(this) => this.max(),
                Self::I8(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
                Self::I6(this) => this.min(),
                Self::I7(this) => this.min(),
                Self::I8(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
                Self::I6(this) => this.max_by_key(f),
                Self::I7(this) => this.max_by_key(f),
                Self::I8(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
                Self::I6(this) => this.max_by(compare),
                Self::I7(this) => this.max_by(compare),
                Self::I8(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
                Self::I6(this) => this.min_by_key(f),
                Self::I7(this) => this.min_by_key(f),
                Self::I8(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
                Self::I6(this) => this.min_by(compare),
                Self::I7(this) => this.min_by(compare),
                Self::I8(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
                Self::I6(this) => this.sum(),
                Self::I7(this) => this.sum(),
                Self::I8(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
                Self::I6(this) => this.product(),
                Self::I7(this) => this.product(),
                Self::I8(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
                Self::I6(this) => this.cmp(other),
                Self::I7(this) => this.cmp(other),
                Self::I8(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
                Self::I6(this) => this.partial_cmp(other),
                Self::I7(this) => this.partial_cmp(other),
                Self::I8(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
                Self::I6(this) => this.eq(other),
                Self::I7(this) => this.eq(other),
                Self::I8(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
                Self::I6(this) => this.ne(other),
                Self::I7(this) => this.ne(other),
                Self::I8(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
                Self::I6(this) => this.lt(other),
                Self::I7(this) => this.lt(other),
                Self::I8(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
                Self::I6(this) => this.le(other),
                Self::I7(this) => this.le(other),
                Self::I8(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
                Self::I6(this) => this.gt(other),
                Self::I7(this) => this.gt(other),
                Self::I8(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
                Self::I6(this) => this.ge(other),
                Self::I7(this) => this.ge(other),
                Self::I8(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8> DoubleEndedIterator
        for Iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
        I6: DoubleEndedIterator<Item = Item>,
        I7: DoubleEndedIterator<Item = Item>,
        I8: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
                Self::I6(this) => this.next_back(),
                Self::I7(this) => this.next_back(),
                Self::I8(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
                Self::I6(this) => this.nth_back(n),
                Self::I7(this) => this.nth_back(n),
                Self::I8(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
                Self::I6(this) => this.rfind(predicate),
                Self::I7(this) => this.rfind(predicate),
                Self::I8(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
                Self::I6(this) => this.rfold(init, f),
                Self::I7(this) => this.rfold(init, f),
                Self::I8(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8> ExactSizeIterator
        for Iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
        I6: ExactSizeIterator<Item = Item>,
        I7: ExactSizeIterator<Item = Item>,
        I8: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
                Self::I6(this) => this.len(),
                Self::I7(this) => this.len(),
                Self::I8(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8> FusedIterator
        for Iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
        I6: FusedIterator<Item = Item>,
        I7: FusedIterator<Item = Item>,
        I8: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter10 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
        I6(I6),
        I7(I7),
        I8(I8),
        I9(I9),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8, I9> {
            Iter10::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8, I9> {
            Iter10::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8, I9> {
            Iter10::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8, I9> {
            Iter10::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8, I9> {
            Iter10::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8, I9> {
            Iter10::I5(self.into_iter())
        }
    }
    pub trait IntoIter6: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter6 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8, I9> {
            Iter10::I6(self.into_iter())
        }
    }
    pub trait IntoIter7: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8, I9>;
    }
    impl<T: IntoIterator> IntoIter7 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8, I9> {
            Iter10::I7(self.into_iter())
        }
    }
    pub trait IntoIter8: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter, I9>;
    }
    impl<T: IntoIterator> IntoIter8 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7, I9>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter, I9> {
            Iter10::I8(self.into_iter())
        }
    }
    pub trait IntoIter9: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter9 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8>(
            self,
        ) -> Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, Self::IntoIter> {
            Iter10::I9(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9> Iterator
        for Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
        I6: Iterator<Item = Item>,
        I7: Iterator<Item = Item>,
        I8: Iterator<Item = Item>,
        I9: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
                Self::I6(this) => this.next(),
                Self::I7(this) => this.next(),
                Self::I8(this) => this.next(),
                Self::I9(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
                Self::I6(this) => this.size_hint(),
                Self::I7(this) => this.size_hint(),
                Self::I8(this) => this.size_hint(),
                Self::I9(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
                Self::I6(this) => this.count(),
                Self::I7(this) => this.count(),
                Self::I8(this) => this.count(),
                Self::I9(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
                Self::I6(this) => this.last(),
                Self::I7(this) => this.last(),
                Self::I8(this) => this.last(),
                Self::I9(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
                Self::I6(this) => this.nth(n),
                Self::I7(this) => this.nth(n),
                Self::I8(this) => this.nth(n),
                Self::I9(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
                Self::I6(this) => this.for_each(f),
                Self::I7(this) => this.for_each(f),
                Self::I8(this) => this.for_each(f),
                Self::I9(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
                Self::I6(this) => this.collect(),
                Self::I7(this) => this.collect(),
                Self::I8(this) => this.collect(),
                Self::I9(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
                Self::I6(this) => this.partition(f),
                Self::I7(this) => this.partition(f),
                Self::I8(this) => this.partition(f),
                Self::I9(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
                Self::I6(this) => this.fold(init, f),
                Self::I7(this) => this.fold(init, f),
                Self::I8(this) => this.fold(init, f),
                Self::I9(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
                Self::I6(this) => this.reduce(f),
                Self::I7(this) => this.reduce(f),
                Self::I8(this) => this.reduce(f),
                Self::I9(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
                Self::I6(this) => this.all(f),
                Self::I7(this) => this.all(f),
                Self::I8(this) => this.all(f),
                Self::I9(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
                Self::I6(this) => this.any(f),
                Self::I7(this) => this.any(f),
                Self::I8(this) => this.any(f),
                Self::I9(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
                Self::I6(this) => this.find(predicate),
                Self::I7(this) => this.find(predicate),
                Self::I8(this) => this.find(predicate),
                Self::I9(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
                Self::I6(this) => this.find_map(f),
                Self::I7(this) => this.find_map(f),
                Self::I8(this) => this.find_map(f),
                Self::I9(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
                Self::I6(this) => this.position(predicate),
                Self::I7(this) => this.position(predicate),
                Self::I8(this) => this.position(predicate),
                Self::I9(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
                Self::I6(this) => this.max(),
                Self::I7(this) => this.max(),
                Self::I8(this) => this.max(),
                Self::I9(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
                Self::I6(this) => this.min(),
                Self::I7(this) => this.min(),
                Self::I8(this) => this.min(),
                Self::I9(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
                Self::I6(this) => this.max_by_key(f),
                Self::I7(this) => this.max_by_key(f),
                Self::I8(this) => this.max_by_key(f),
                Self::I9(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
                Self::I6(this) => this.max_by(compare),
                Self::I7(this) => this.max_by(compare),
                Self::I8(this) => this.max_by(compare),
                Self::I9(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
                Self::I6(this) => this.min_by_key(f),
                Self::I7(this) => this.min_by_key(f),
                Self::I8(this) => this.min_by_key(f),
                Self::I9(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
                Self::I6(this) => this.min_by(compare),
                Self::I7(this) => this.min_by(compare),
                Self::I8(this) => this.min_by(compare),
                Self::I9(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
                Self::I6(this) => this.sum(),
                Self::I7(this) => this.sum(),
                Self::I8(this) => this.sum(),
                Self::I9(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
                Self::I6(this) => this.product(),
                Self::I7(this) => this.product(),
                Self::I8(this) => this.product(),
                Self::I9(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
                Self::I6(this) => this.cmp(other),
                Self::I7(this) => this.cmp(other),
                Self::I8(this) => this.cmp(other),
                Self::I9(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
                Self::I6(this) => this.partial_cmp(other),
                Self::I7(this) => this.partial_cmp(other),
                Self::I8(this) => this.partial_cmp(other),
                Self::I9(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
                Self::I6(this) => this.eq(other),
                Self::I7(this) => this.eq(other),
                Self::I8(this) => this.eq(other),
                Self::I9(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
                Self::I6(this) => this.ne(other),
                Self::I7(this) => this.ne(other),
                Self::I8(this) => this.ne(other),
                Self::I9(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
                Self::I6(this) => this.lt(other),
                Self::I7(this) => this.lt(other),
                Self::I8(this) => this.lt(other),
                Self::I9(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
                Self::I6(this) => this.le(other),
                Self::I7(this) => this.le(other),
                Self::I8(this) => this.le(other),
                Self::I9(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
                Self::I6(this) => this.gt(other),
                Self::I7(this) => this.gt(other),
                Self::I8(this) => this.gt(other),
                Self::I9(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
                Self::I6(this) => this.ge(other),
                Self::I7(this) => this.ge(other),
                Self::I8(this) => this.ge(other),
                Self::I9(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9> DoubleEndedIterator
        for Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
        I6: DoubleEndedIterator<Item = Item>,
        I7: DoubleEndedIterator<Item = Item>,
        I8: DoubleEndedIterator<Item = Item>,
        I9: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
                Self::I6(this) => this.next_back(),
                Self::I7(this) => this.next_back(),
                Self::I8(this) => this.next_back(),
                Self::I9(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
                Self::I6(this) => this.nth_back(n),
                Self::I7(this) => this.nth_back(n),
                Self::I8(this) => this.nth_back(n),
                Self::I9(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
                Self::I6(this) => this.rfind(predicate),
                Self::I7(this) => this.rfind(predicate),
                Self::I8(this) => this.rfind(predicate),
                Self::I9(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
                Self::I6(this) => this.rfold(init, f),
                Self::I7(this) => this.rfold(init, f),
                Self::I8(this) => this.rfold(init, f),
                Self::I9(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9> ExactSizeIterator
        for Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
        I6: ExactSizeIterator<Item = Item>,
        I7: ExactSizeIterator<Item = Item>,
        I8: ExactSizeIterator<Item = Item>,
        I9: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
                Self::I6(this) => this.len(),
                Self::I7(this) => this.len(),
                Self::I8(this) => this.len(),
                Self::I9(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9> FusedIterator
        for Iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
        I6: FusedIterator<Item = Item>,
        I7: FusedIterator<Item = Item>,
        I8: FusedIterator<Item = Item>,
        I9: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter11 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
        I6(I6),
        I7(I7),
        I8(I8),
        I9(I9),
        I10(I10),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10> {
            Iter11::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8, I9, I10> {
            Iter11::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8, I9, I10> {
            Iter11::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8, I9, I10> {
            Iter11::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8, I9, I10> {
            Iter11::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8, I9, I10> {
            Iter11::I5(self.into_iter())
        }
    }
    pub trait IntoIter6: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter6 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8, I9, I10> {
            Iter11::I6(self.into_iter())
        }
    }
    pub trait IntoIter7: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter7 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8, I9, I10> {
            Iter11::I7(self.into_iter())
        }
    }
    pub trait IntoIter8: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter, I9, I10>;
    }
    impl<T: IntoIterator> IntoIter8 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7, I9, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter, I9, I10> {
            Iter11::I8(self.into_iter())
        }
    }
    pub trait IntoIter9: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, Self::IntoIter, I10>;
    }
    impl<T: IntoIterator> IntoIter9 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8, I10>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, Self::IntoIter, I10> {
            Iter11::I9(self.into_iter())
        }
    }
    pub trait IntoIter10: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter10 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9>(
            self,
        ) -> Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, Self::IntoIter> {
            Iter11::I10(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10> Iterator
        for Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
        I6: Iterator<Item = Item>,
        I7: Iterator<Item = Item>,
        I8: Iterator<Item = Item>,
        I9: Iterator<Item = Item>,
        I10: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
                Self::I6(this) => this.next(),
                Self::I7(this) => this.next(),
                Self::I8(this) => this.next(),
                Self::I9(this) => this.next(),
                Self::I10(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
                Self::I6(this) => this.size_hint(),
                Self::I7(this) => this.size_hint(),
                Self::I8(this) => this.size_hint(),
                Self::I9(this) => this.size_hint(),
                Self::I10(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
                Self::I6(this) => this.count(),
                Self::I7(this) => this.count(),
                Self::I8(this) => this.count(),
                Self::I9(this) => this.count(),
                Self::I10(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
                Self::I6(this) => this.last(),
                Self::I7(this) => this.last(),
                Self::I8(this) => this.last(),
                Self::I9(this) => this.last(),
                Self::I10(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
                Self::I6(this) => this.nth(n),
                Self::I7(this) => this.nth(n),
                Self::I8(this) => this.nth(n),
                Self::I9(this) => this.nth(n),
                Self::I10(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
                Self::I6(this) => this.for_each(f),
                Self::I7(this) => this.for_each(f),
                Self::I8(this) => this.for_each(f),
                Self::I9(this) => this.for_each(f),
                Self::I10(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
                Self::I6(this) => this.collect(),
                Self::I7(this) => this.collect(),
                Self::I8(this) => this.collect(),
                Self::I9(this) => this.collect(),
                Self::I10(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
                Self::I6(this) => this.partition(f),
                Self::I7(this) => this.partition(f),
                Self::I8(this) => this.partition(f),
                Self::I9(this) => this.partition(f),
                Self::I10(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
                Self::I6(this) => this.fold(init, f),
                Self::I7(this) => this.fold(init, f),
                Self::I8(this) => this.fold(init, f),
                Self::I9(this) => this.fold(init, f),
                Self::I10(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
                Self::I6(this) => this.reduce(f),
                Self::I7(this) => this.reduce(f),
                Self::I8(this) => this.reduce(f),
                Self::I9(this) => this.reduce(f),
                Self::I10(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
                Self::I6(this) => this.all(f),
                Self::I7(this) => this.all(f),
                Self::I8(this) => this.all(f),
                Self::I9(this) => this.all(f),
                Self::I10(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
                Self::I6(this) => this.any(f),
                Self::I7(this) => this.any(f),
                Self::I8(this) => this.any(f),
                Self::I9(this) => this.any(f),
                Self::I10(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
                Self::I6(this) => this.find(predicate),
                Self::I7(this) => this.find(predicate),
                Self::I8(this) => this.find(predicate),
                Self::I9(this) => this.find(predicate),
                Self::I10(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
                Self::I6(this) => this.find_map(f),
                Self::I7(this) => this.find_map(f),
                Self::I8(this) => this.find_map(f),
                Self::I9(this) => this.find_map(f),
                Self::I10(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
                Self::I6(this) => this.position(predicate),
                Self::I7(this) => this.position(predicate),
                Self::I8(this) => this.position(predicate),
                Self::I9(this) => this.position(predicate),
                Self::I10(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
                Self::I6(this) => this.max(),
                Self::I7(this) => this.max(),
                Self::I8(this) => this.max(),
                Self::I9(this) => this.max(),
                Self::I10(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
                Self::I6(this) => this.min(),
                Self::I7(this) => this.min(),
                Self::I8(this) => this.min(),
                Self::I9(this) => this.min(),
                Self::I10(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
                Self::I6(this) => this.max_by_key(f),
                Self::I7(this) => this.max_by_key(f),
                Self::I8(this) => this.max_by_key(f),
                Self::I9(this) => this.max_by_key(f),
                Self::I10(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
                Self::I6(this) => this.max_by(compare),
                Self::I7(this) => this.max_by(compare),
                Self::I8(this) => this.max_by(compare),
                Self::I9(this) => this.max_by(compare),
                Self::I10(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
                Self::I6(this) => this.min_by_key(f),
                Self::I7(this) => this.min_by_key(f),
                Self::I8(this) => this.min_by_key(f),
                Self::I9(this) => this.min_by_key(f),
                Self::I10(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
                Self::I6(this) => this.min_by(compare),
                Self::I7(this) => this.min_by(compare),
                Self::I8(this) => this.min_by(compare),
                Self::I9(this) => this.min_by(compare),
                Self::I10(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
                Self::I6(this) => this.sum(),
                Self::I7(this) => this.sum(),
                Self::I8(this) => this.sum(),
                Self::I9(this) => this.sum(),
                Self::I10(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
                Self::I6(this) => this.product(),
                Self::I7(this) => this.product(),
                Self::I8(this) => this.product(),
                Self::I9(this) => this.product(),
                Self::I10(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
                Self::I6(this) => this.cmp(other),
                Self::I7(this) => this.cmp(other),
                Self::I8(this) => this.cmp(other),
                Self::I9(this) => this.cmp(other),
                Self::I10(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
                Self::I6(this) => this.partial_cmp(other),
                Self::I7(this) => this.partial_cmp(other),
                Self::I8(this) => this.partial_cmp(other),
                Self::I9(this) => this.partial_cmp(other),
                Self::I10(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
                Self::I6(this) => this.eq(other),
                Self::I7(this) => this.eq(other),
                Self::I8(this) => this.eq(other),
                Self::I9(this) => this.eq(other),
                Self::I10(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
                Self::I6(this) => this.ne(other),
                Self::I7(this) => this.ne(other),
                Self::I8(this) => this.ne(other),
                Self::I9(this) => this.ne(other),
                Self::I10(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
                Self::I6(this) => this.lt(other),
                Self::I7(this) => this.lt(other),
                Self::I8(this) => this.lt(other),
                Self::I9(this) => this.lt(other),
                Self::I10(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
                Self::I6(this) => this.le(other),
                Self::I7(this) => this.le(other),
                Self::I8(this) => this.le(other),
                Self::I9(this) => this.le(other),
                Self::I10(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
                Self::I6(this) => this.gt(other),
                Self::I7(this) => this.gt(other),
                Self::I8(this) => this.gt(other),
                Self::I9(this) => this.gt(other),
                Self::I10(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
                Self::I6(this) => this.ge(other),
                Self::I7(this) => this.ge(other),
                Self::I8(this) => this.ge(other),
                Self::I9(this) => this.ge(other),
                Self::I10(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10> DoubleEndedIterator
        for Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
        I6: DoubleEndedIterator<Item = Item>,
        I7: DoubleEndedIterator<Item = Item>,
        I8: DoubleEndedIterator<Item = Item>,
        I9: DoubleEndedIterator<Item = Item>,
        I10: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
                Self::I6(this) => this.next_back(),
                Self::I7(this) => this.next_back(),
                Self::I8(this) => this.next_back(),
                Self::I9(this) => this.next_back(),
                Self::I10(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
                Self::I6(this) => this.nth_back(n),
                Self::I7(this) => this.nth_back(n),
                Self::I8(this) => this.nth_back(n),
                Self::I9(this) => this.nth_back(n),
                Self::I10(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
                Self::I6(this) => this.rfind(predicate),
                Self::I7(this) => this.rfind(predicate),
                Self::I8(this) => this.rfind(predicate),
                Self::I9(this) => this.rfind(predicate),
                Self::I10(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
                Self::I6(this) => this.rfold(init, f),
                Self::I7(this) => this.rfold(init, f),
                Self::I8(this) => this.rfold(init, f),
                Self::I9(this) => this.rfold(init, f),
                Self::I10(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10> ExactSizeIterator
        for Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
        I6: ExactSizeIterator<Item = Item>,
        I7: ExactSizeIterator<Item = Item>,
        I8: ExactSizeIterator<Item = Item>,
        I9: ExactSizeIterator<Item = Item>,
        I10: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
                Self::I6(this) => this.len(),
                Self::I7(this) => this.len(),
                Self::I8(this) => this.len(),
                Self::I9(this) => this.len(),
                Self::I10(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10> FusedIterator
        for Iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
        I6: FusedIterator<Item = Item>,
        I7: FusedIterator<Item = Item>,
        I8: FusedIterator<Item = Item>,
        I9: FusedIterator<Item = Item>,
        I10: FusedIterator<Item = Item>,
    {
    }
}
pub mod iter12 {
    use super::*;
    use core::iter::FusedIterator;
    #[derive(Debug, Clone, Copy)]
    pub enum Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> {
        I0(I0),
        I1(I1),
        I2(I2),
        I3(I3),
        I4(I4),
        I5(I5),
        I6(I6),
        I7(I7),
        I8(I8),
        I9(I9),
        I10(I10),
        I11(I11),
    }
    pub trait IntoIter0: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter0 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter0<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<Self::IntoIter, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> {
            Iter12::I0(self.into_iter())
        }
    }
    pub trait IntoIter1: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter1 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter1<I0, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, Self::IntoIter, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> {
            Iter12::I1(self.into_iter())
        }
    }
    pub trait IntoIter2: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter2 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter2<I0, I1, I3, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, Self::IntoIter, I3, I4, I5, I6, I7, I8, I9, I10, I11> {
            Iter12::I2(self.into_iter())
        }
    }
    pub trait IntoIter3: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter3 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter3<I0, I1, I2, I4, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, Self::IntoIter, I4, I5, I6, I7, I8, I9, I10, I11> {
            Iter12::I3(self.into_iter())
        }
    }
    pub trait IntoIter4: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter4 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter4<I0, I1, I2, I3, I5, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, Self::IntoIter, I5, I6, I7, I8, I9, I10, I11> {
            Iter12::I4(self.into_iter())
        }
    }
    pub trait IntoIter5: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter5 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter5<I0, I1, I2, I3, I4, I6, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, Self::IntoIter, I6, I7, I8, I9, I10, I11> {
            Iter12::I5(self.into_iter())
        }
    }
    pub trait IntoIter6: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter6 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter6<I0, I1, I2, I3, I4, I5, I7, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, Self::IntoIter, I7, I8, I9, I10, I11> {
            Iter12::I6(self.into_iter())
        }
    }
    pub trait IntoIter7: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter7 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter7<I0, I1, I2, I3, I4, I5, I6, I8, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, Self::IntoIter, I8, I9, I10, I11> {
            Iter12::I7(self.into_iter())
        }
    }
    pub trait IntoIter8: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter, I9, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter8 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter8<I0, I1, I2, I3, I4, I5, I6, I7, I9, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, Self::IntoIter, I9, I10, I11> {
            Iter12::I8(self.into_iter())
        }
    }
    pub trait IntoIter9: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, Self::IntoIter, I10, I11>;
    }
    impl<T: IntoIterator> IntoIter9 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter9<I0, I1, I2, I3, I4, I5, I6, I7, I8, I10, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, Self::IntoIter, I10, I11> {
            Iter12::I9(self.into_iter())
        }
    }
    pub trait IntoIter10: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, Self::IntoIter, I11>;
    }
    impl<T: IntoIterator> IntoIter10 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter10<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I11>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, Self::IntoIter, I11> {
            Iter12::I10(self.into_iter())
        }
    }
    pub trait IntoIter11: IntoIterator {
        #[allow(clippy::type_complexity)]
        fn into_iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, Self::IntoIter>;
    }
    impl<T: IntoIterator> IntoIter11 for T {
        #[allow(clippy::type_complexity)]
        fn into_iter11<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>(
            self,
        ) -> Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, Self::IntoIter> {
            Iter12::I11(self.into_iter())
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> Iterator
        for Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>
    where
        I0: Iterator<Item = Item>,
        I1: Iterator<Item = Item>,
        I2: Iterator<Item = Item>,
        I3: Iterator<Item = Item>,
        I4: Iterator<Item = Item>,
        I5: Iterator<Item = Item>,
        I6: Iterator<Item = Item>,
        I7: Iterator<Item = Item>,
        I8: Iterator<Item = Item>,
        I9: Iterator<Item = Item>,
        I10: Iterator<Item = Item>,
        I11: Iterator<Item = Item>,
    {
        type Item = Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next(),
                Self::I1(this) => this.next(),
                Self::I2(this) => this.next(),
                Self::I3(this) => this.next(),
                Self::I4(this) => this.next(),
                Self::I5(this) => this.next(),
                Self::I6(this) => this.next(),
                Self::I7(this) => this.next(),
                Self::I8(this) => this.next(),
                Self::I9(this) => this.next(),
                Self::I10(this) => this.next(),
                Self::I11(this) => this.next(),
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::I0(this) => this.size_hint(),
                Self::I1(this) => this.size_hint(),
                Self::I2(this) => this.size_hint(),
                Self::I3(this) => this.size_hint(),
                Self::I4(this) => this.size_hint(),
                Self::I5(this) => this.size_hint(),
                Self::I6(this) => this.size_hint(),
                Self::I7(this) => this.size_hint(),
                Self::I8(this) => this.size_hint(),
                Self::I9(this) => this.size_hint(),
                Self::I10(this) => this.size_hint(),
                Self::I11(this) => this.size_hint(),
            }
        }
        fn count(self) -> usize
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.count(),
                Self::I1(this) => this.count(),
                Self::I2(this) => this.count(),
                Self::I3(this) => this.count(),
                Self::I4(this) => this.count(),
                Self::I5(this) => this.count(),
                Self::I6(this) => this.count(),
                Self::I7(this) => this.count(),
                Self::I8(this) => this.count(),
                Self::I9(this) => this.count(),
                Self::I10(this) => this.count(),
                Self::I11(this) => this.count(),
            }
        }
        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.last(),
                Self::I1(this) => this.last(),
                Self::I2(this) => this.last(),
                Self::I3(this) => this.last(),
                Self::I4(this) => this.last(),
                Self::I5(this) => this.last(),
                Self::I6(this) => this.last(),
                Self::I7(this) => this.last(),
                Self::I8(this) => this.last(),
                Self::I9(this) => this.last(),
                Self::I10(this) => this.last(),
                Self::I11(this) => this.last(),
            }
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth(n),
                Self::I1(this) => this.nth(n),
                Self::I2(this) => this.nth(n),
                Self::I3(this) => this.nth(n),
                Self::I4(this) => this.nth(n),
                Self::I5(this) => this.nth(n),
                Self::I6(this) => this.nth(n),
                Self::I7(this) => this.nth(n),
                Self::I8(this) => this.nth(n),
                Self::I9(this) => this.nth(n),
                Self::I10(this) => this.nth(n),
                Self::I11(this) => this.nth(n),
            }
        }
        fn for_each<F>(self, f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item),
        {
            match self {
                Self::I0(this) => this.for_each(f),
                Self::I1(this) => this.for_each(f),
                Self::I2(this) => this.for_each(f),
                Self::I3(this) => this.for_each(f),
                Self::I4(this) => this.for_each(f),
                Self::I5(this) => this.for_each(f),
                Self::I6(this) => this.for_each(f),
                Self::I7(this) => this.for_each(f),
                Self::I8(this) => this.for_each(f),
                Self::I9(this) => this.for_each(f),
                Self::I10(this) => this.for_each(f),
                Self::I11(this) => this.for_each(f),
            }
        }
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.collect(),
                Self::I1(this) => this.collect(),
                Self::I2(this) => this.collect(),
                Self::I3(this) => this.collect(),
                Self::I4(this) => this.collect(),
                Self::I5(this) => this.collect(),
                Self::I6(this) => this.collect(),
                Self::I7(this) => this.collect(),
                Self::I8(this) => this.collect(),
                Self::I9(this) => this.collect(),
                Self::I10(this) => this.collect(),
                Self::I11(this) => this.collect(),
            }
        }
        fn partition<B, F>(self, f: F) -> (B, B)
        where
            Self: Sized,
            B: Default + Extend<Self::Item>,
            F: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.partition(f),
                Self::I1(this) => this.partition(f),
                Self::I2(this) => this.partition(f),
                Self::I3(this) => this.partition(f),
                Self::I4(this) => this.partition(f),
                Self::I5(this) => this.partition(f),
                Self::I6(this) => this.partition(f),
                Self::I7(this) => this.partition(f),
                Self::I8(this) => this.partition(f),
                Self::I9(this) => this.partition(f),
                Self::I10(this) => this.partition(f),
                Self::I11(this) => this.partition(f),
            }
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.fold(init, f),
                Self::I1(this) => this.fold(init, f),
                Self::I2(this) => this.fold(init, f),
                Self::I3(this) => this.fold(init, f),
                Self::I4(this) => this.fold(init, f),
                Self::I5(this) => this.fold(init, f),
                Self::I6(this) => this.fold(init, f),
                Self::I7(this) => this.fold(init, f),
                Self::I8(this) => this.fold(init, f),
                Self::I9(this) => this.fold(init, f),
                Self::I10(this) => this.fold(init, f),
                Self::I11(this) => this.fold(init, f),
            }
        }
        fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        {
            match self {
                Self::I0(this) => this.reduce(f),
                Self::I1(this) => this.reduce(f),
                Self::I2(this) => this.reduce(f),
                Self::I3(this) => this.reduce(f),
                Self::I4(this) => this.reduce(f),
                Self::I5(this) => this.reduce(f),
                Self::I6(this) => this.reduce(f),
                Self::I7(this) => this.reduce(f),
                Self::I8(this) => this.reduce(f),
                Self::I9(this) => this.reduce(f),
                Self::I10(this) => this.reduce(f),
                Self::I11(this) => this.reduce(f),
            }
        }
        fn all<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.all(f),
                Self::I1(this) => this.all(f),
                Self::I2(this) => this.all(f),
                Self::I3(this) => this.all(f),
                Self::I4(this) => this.all(f),
                Self::I5(this) => this.all(f),
                Self::I6(this) => this.all(f),
                Self::I7(this) => this.all(f),
                Self::I8(this) => this.all(f),
                Self::I9(this) => this.all(f),
                Self::I10(this) => this.all(f),
                Self::I11(this) => this.all(f),
            }
        }
        fn any<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.any(f),
                Self::I1(this) => this.any(f),
                Self::I2(this) => this.any(f),
                Self::I3(this) => this.any(f),
                Self::I4(this) => this.any(f),
                Self::I5(this) => this.any(f),
                Self::I6(this) => this.any(f),
                Self::I7(this) => this.any(f),
                Self::I8(this) => this.any(f),
                Self::I9(this) => this.any(f),
                Self::I10(this) => this.any(f),
                Self::I11(this) => this.any(f),
            }
        }
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.find(predicate),
                Self::I1(this) => this.find(predicate),
                Self::I2(this) => this.find(predicate),
                Self::I3(this) => this.find(predicate),
                Self::I4(this) => this.find(predicate),
                Self::I5(this) => this.find(predicate),
                Self::I6(this) => this.find(predicate),
                Self::I7(this) => this.find(predicate),
                Self::I8(this) => this.find(predicate),
                Self::I9(this) => this.find(predicate),
                Self::I10(this) => this.find(predicate),
                Self::I11(this) => this.find(predicate),
            }
        }
        fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> Option<B>,
        {
            match self {
                Self::I0(this) => this.find_map(f),
                Self::I1(this) => this.find_map(f),
                Self::I2(this) => this.find_map(f),
                Self::I3(this) => this.find_map(f),
                Self::I4(this) => this.find_map(f),
                Self::I5(this) => this.find_map(f),
                Self::I6(this) => this.find_map(f),
                Self::I7(this) => this.find_map(f),
                Self::I8(this) => this.find_map(f),
                Self::I9(this) => this.find_map(f),
                Self::I10(this) => this.find_map(f),
                Self::I11(this) => this.find_map(f),
            }
        }
        fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: Sized,
            P: FnMut(Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.position(predicate),
                Self::I1(this) => this.position(predicate),
                Self::I2(this) => this.position(predicate),
                Self::I3(this) => this.position(predicate),
                Self::I4(this) => this.position(predicate),
                Self::I5(this) => this.position(predicate),
                Self::I6(this) => this.position(predicate),
                Self::I7(this) => this.position(predicate),
                Self::I8(this) => this.position(predicate),
                Self::I9(this) => this.position(predicate),
                Self::I10(this) => this.position(predicate),
                Self::I11(this) => this.position(predicate),
            }
        }
        fn max(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.max(),
                Self::I1(this) => this.max(),
                Self::I2(this) => this.max(),
                Self::I3(this) => this.max(),
                Self::I4(this) => this.max(),
                Self::I5(this) => this.max(),
                Self::I6(this) => this.max(),
                Self::I7(this) => this.max(),
                Self::I8(this) => this.max(),
                Self::I9(this) => this.max(),
                Self::I10(this) => this.max(),
                Self::I11(this) => this.max(),
            }
        }
        fn min(self) -> Option<Self::Item>
        where
            Self: Sized,
            Self::Item: Ord,
        {
            match self {
                Self::I0(this) => this.min(),
                Self::I1(this) => this.min(),
                Self::I2(this) => this.min(),
                Self::I3(this) => this.min(),
                Self::I4(this) => this.min(),
                Self::I5(this) => this.min(),
                Self::I6(this) => this.min(),
                Self::I7(this) => this.min(),
                Self::I8(this) => this.min(),
                Self::I9(this) => this.min(),
                Self::I10(this) => this.min(),
                Self::I11(this) => this.min(),
            }
        }
        fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.max_by_key(f),
                Self::I1(this) => this.max_by_key(f),
                Self::I2(this) => this.max_by_key(f),
                Self::I3(this) => this.max_by_key(f),
                Self::I4(this) => this.max_by_key(f),
                Self::I5(this) => this.max_by_key(f),
                Self::I6(this) => this.max_by_key(f),
                Self::I7(this) => this.max_by_key(f),
                Self::I8(this) => this.max_by_key(f),
                Self::I9(this) => this.max_by_key(f),
                Self::I10(this) => this.max_by_key(f),
                Self::I11(this) => this.max_by_key(f),
            }
        }
        fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.max_by(compare),
                Self::I1(this) => this.max_by(compare),
                Self::I2(this) => this.max_by(compare),
                Self::I3(this) => this.max_by(compare),
                Self::I4(this) => this.max_by(compare),
                Self::I5(this) => this.max_by(compare),
                Self::I6(this) => this.max_by(compare),
                Self::I7(this) => this.max_by(compare),
                Self::I8(this) => this.max_by(compare),
                Self::I9(this) => this.max_by(compare),
                Self::I10(this) => this.max_by(compare),
                Self::I11(this) => this.max_by(compare),
            }
        }
        fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.min_by_key(f),
                Self::I1(this) => this.min_by_key(f),
                Self::I2(this) => this.min_by_key(f),
                Self::I3(this) => this.min_by_key(f),
                Self::I4(this) => this.min_by_key(f),
                Self::I5(this) => this.min_by_key(f),
                Self::I6(this) => this.min_by_key(f),
                Self::I7(this) => this.min_by_key(f),
                Self::I8(this) => this.min_by_key(f),
                Self::I9(this) => this.min_by_key(f),
                Self::I10(this) => this.min_by_key(f),
                Self::I11(this) => this.min_by_key(f),
            }
        }
        fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            match self {
                Self::I0(this) => this.min_by(compare),
                Self::I1(this) => this.min_by(compare),
                Self::I2(this) => this.min_by(compare),
                Self::I3(this) => this.min_by(compare),
                Self::I4(this) => this.min_by(compare),
                Self::I5(this) => this.min_by(compare),
                Self::I6(this) => this.min_by(compare),
                Self::I7(this) => this.min_by(compare),
                Self::I8(this) => this.min_by(compare),
                Self::I9(this) => this.min_by(compare),
                Self::I10(this) => this.min_by(compare),
                Self::I11(this) => this.min_by(compare),
            }
        }
        fn sum<S>(self) -> S
        where
            Self: Sized,
            S: Sum<Self::Item>,
        {
            match self {
                Self::I0(this) => this.sum(),
                Self::I1(this) => this.sum(),
                Self::I2(this) => this.sum(),
                Self::I3(this) => this.sum(),
                Self::I4(this) => this.sum(),
                Self::I5(this) => this.sum(),
                Self::I6(this) => this.sum(),
                Self::I7(this) => this.sum(),
                Self::I8(this) => this.sum(),
                Self::I9(this) => this.sum(),
                Self::I10(this) => this.sum(),
                Self::I11(this) => this.sum(),
            }
        }
        fn product<P>(self) -> P
        where
            Self: Sized,
            P: Product<Self::Item>,
        {
            match self {
                Self::I0(this) => this.product(),
                Self::I1(this) => this.product(),
                Self::I2(this) => this.product(),
                Self::I3(this) => this.product(),
                Self::I4(this) => this.product(),
                Self::I5(this) => this.product(),
                Self::I6(this) => this.product(),
                Self::I7(this) => this.product(),
                Self::I8(this) => this.product(),
                Self::I9(this) => this.product(),
                Self::I10(this) => this.product(),
                Self::I11(this) => this.product(),
            }
        }
        fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.cmp(other),
                Self::I1(this) => this.cmp(other),
                Self::I2(this) => this.cmp(other),
                Self::I3(this) => this.cmp(other),
                Self::I4(this) => this.cmp(other),
                Self::I5(this) => this.cmp(other),
                Self::I6(this) => this.cmp(other),
                Self::I7(this) => this.cmp(other),
                Self::I8(this) => this.cmp(other),
                Self::I9(this) => this.cmp(other),
                Self::I10(this) => this.cmp(other),
                Self::I11(this) => this.cmp(other),
            }
        }
        fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.partial_cmp(other),
                Self::I1(this) => this.partial_cmp(other),
                Self::I2(this) => this.partial_cmp(other),
                Self::I3(this) => this.partial_cmp(other),
                Self::I4(this) => this.partial_cmp(other),
                Self::I5(this) => this.partial_cmp(other),
                Self::I6(this) => this.partial_cmp(other),
                Self::I7(this) => this.partial_cmp(other),
                Self::I8(this) => this.partial_cmp(other),
                Self::I9(this) => this.partial_cmp(other),
                Self::I10(this) => this.partial_cmp(other),
                Self::I11(this) => this.partial_cmp(other),
            }
        }
        fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.eq(other),
                Self::I1(this) => this.eq(other),
                Self::I2(this) => this.eq(other),
                Self::I3(this) => this.eq(other),
                Self::I4(this) => this.eq(other),
                Self::I5(this) => this.eq(other),
                Self::I6(this) => this.eq(other),
                Self::I7(this) => this.eq(other),
                Self::I8(this) => this.eq(other),
                Self::I9(this) => this.eq(other),
                Self::I10(this) => this.eq(other),
                Self::I11(this) => this.eq(other),
            }
        }
        fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ne(other),
                Self::I1(this) => this.ne(other),
                Self::I2(this) => this.ne(other),
                Self::I3(this) => this.ne(other),
                Self::I4(this) => this.ne(other),
                Self::I5(this) => this.ne(other),
                Self::I6(this) => this.ne(other),
                Self::I7(this) => this.ne(other),
                Self::I8(this) => this.ne(other),
                Self::I9(this) => this.ne(other),
                Self::I10(this) => this.ne(other),
                Self::I11(this) => this.ne(other),
            }
        }
        fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.lt(other),
                Self::I1(this) => this.lt(other),
                Self::I2(this) => this.lt(other),
                Self::I3(this) => this.lt(other),
                Self::I4(this) => this.lt(other),
                Self::I5(this) => this.lt(other),
                Self::I6(this) => this.lt(other),
                Self::I7(this) => this.lt(other),
                Self::I8(this) => this.lt(other),
                Self::I9(this) => this.lt(other),
                Self::I10(this) => this.lt(other),
                Self::I11(this) => this.lt(other),
            }
        }
        fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.le(other),
                Self::I1(this) => this.le(other),
                Self::I2(this) => this.le(other),
                Self::I3(this) => this.le(other),
                Self::I4(this) => this.le(other),
                Self::I5(this) => this.le(other),
                Self::I6(this) => this.le(other),
                Self::I7(this) => this.le(other),
                Self::I8(this) => this.le(other),
                Self::I9(this) => this.le(other),
                Self::I10(this) => this.le(other),
                Self::I11(this) => this.le(other),
            }
        }
        fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.gt(other),
                Self::I1(this) => this.gt(other),
                Self::I2(this) => this.gt(other),
                Self::I3(this) => this.gt(other),
                Self::I4(this) => this.gt(other),
                Self::I5(this) => this.gt(other),
                Self::I6(this) => this.gt(other),
                Self::I7(this) => this.gt(other),
                Self::I8(this) => this.gt(other),
                Self::I9(this) => this.gt(other),
                Self::I10(this) => this.gt(other),
                Self::I11(this) => this.gt(other),
            }
        }
        fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<I::Item>,
            Self: Sized,
        {
            match self {
                Self::I0(this) => this.ge(other),
                Self::I1(this) => this.ge(other),
                Self::I2(this) => this.ge(other),
                Self::I3(this) => this.ge(other),
                Self::I4(this) => this.ge(other),
                Self::I5(this) => this.ge(other),
                Self::I6(this) => this.ge(other),
                Self::I7(this) => this.ge(other),
                Self::I8(this) => this.ge(other),
                Self::I9(this) => this.ge(other),
                Self::I10(this) => this.ge(other),
                Self::I11(this) => this.ge(other),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> DoubleEndedIterator
        for Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>
    where
        I0: DoubleEndedIterator<Item = Item>,
        I1: DoubleEndedIterator<Item = Item>,
        I2: DoubleEndedIterator<Item = Item>,
        I3: DoubleEndedIterator<Item = Item>,
        I4: DoubleEndedIterator<Item = Item>,
        I5: DoubleEndedIterator<Item = Item>,
        I6: DoubleEndedIterator<Item = Item>,
        I7: DoubleEndedIterator<Item = Item>,
        I8: DoubleEndedIterator<Item = Item>,
        I9: DoubleEndedIterator<Item = Item>,
        I10: DoubleEndedIterator<Item = Item>,
        I11: DoubleEndedIterator<Item = Item>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.next_back(),
                Self::I1(this) => this.next_back(),
                Self::I2(this) => this.next_back(),
                Self::I3(this) => this.next_back(),
                Self::I4(this) => this.next_back(),
                Self::I5(this) => this.next_back(),
                Self::I6(this) => this.next_back(),
                Self::I7(this) => this.next_back(),
                Self::I8(this) => this.next_back(),
                Self::I9(this) => this.next_back(),
                Self::I10(this) => this.next_back(),
                Self::I11(this) => this.next_back(),
            }
        }
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                Self::I0(this) => this.nth_back(n),
                Self::I1(this) => this.nth_back(n),
                Self::I2(this) => this.nth_back(n),
                Self::I3(this) => this.nth_back(n),
                Self::I4(this) => this.nth_back(n),
                Self::I5(this) => this.nth_back(n),
                Self::I6(this) => this.nth_back(n),
                Self::I7(this) => this.nth_back(n),
                Self::I8(this) => this.nth_back(n),
                Self::I9(this) => this.nth_back(n),
                Self::I10(this) => this.nth_back(n),
                Self::I11(this) => this.nth_back(n),
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            match self {
                Self::I0(this) => this.rfind(predicate),
                Self::I1(this) => this.rfind(predicate),
                Self::I2(this) => this.rfind(predicate),
                Self::I3(this) => this.rfind(predicate),
                Self::I4(this) => this.rfind(predicate),
                Self::I5(this) => this.rfind(predicate),
                Self::I6(this) => this.rfind(predicate),
                Self::I7(this) => this.rfind(predicate),
                Self::I8(this) => this.rfind(predicate),
                Self::I9(this) => this.rfind(predicate),
                Self::I10(this) => this.rfind(predicate),
                Self::I11(this) => this.rfind(predicate),
            }
        }
        fn rfold<B, F>(self, init: B, f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B,
        {
            match self {
                Self::I0(this) => this.rfold(init, f),
                Self::I1(this) => this.rfold(init, f),
                Self::I2(this) => this.rfold(init, f),
                Self::I3(this) => this.rfold(init, f),
                Self::I4(this) => this.rfold(init, f),
                Self::I5(this) => this.rfold(init, f),
                Self::I6(this) => this.rfold(init, f),
                Self::I7(this) => this.rfold(init, f),
                Self::I8(this) => this.rfold(init, f),
                Self::I9(this) => this.rfold(init, f),
                Self::I10(this) => this.rfold(init, f),
                Self::I11(this) => this.rfold(init, f),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> ExactSizeIterator
        for Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>
    where
        I0: ExactSizeIterator<Item = Item>,
        I1: ExactSizeIterator<Item = Item>,
        I2: ExactSizeIterator<Item = Item>,
        I3: ExactSizeIterator<Item = Item>,
        I4: ExactSizeIterator<Item = Item>,
        I5: ExactSizeIterator<Item = Item>,
        I6: ExactSizeIterator<Item = Item>,
        I7: ExactSizeIterator<Item = Item>,
        I8: ExactSizeIterator<Item = Item>,
        I9: ExactSizeIterator<Item = Item>,
        I10: ExactSizeIterator<Item = Item>,
        I11: ExactSizeIterator<Item = Item>,
    {
        fn len(&self) -> usize {
            match self {
                Self::I0(this) => this.len(),
                Self::I1(this) => this.len(),
                Self::I2(this) => this.len(),
                Self::I3(this) => this.len(),
                Self::I4(this) => this.len(),
                Self::I5(this) => this.len(),
                Self::I6(this) => this.len(),
                Self::I7(this) => this.len(),
                Self::I8(this) => this.len(),
                Self::I9(this) => this.len(),
                Self::I10(this) => this.len(),
                Self::I11(this) => this.len(),
            }
        }
    }
    impl<Item, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11> FusedIterator
        for Iter12<I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>
    where
        I0: FusedIterator<Item = Item>,
        I1: FusedIterator<Item = Item>,
        I2: FusedIterator<Item = Item>,
        I3: FusedIterator<Item = Item>,
        I4: FusedIterator<Item = Item>,
        I5: FusedIterator<Item = Item>,
        I6: FusedIterator<Item = Item>,
        I7: FusedIterator<Item = Item>,
        I8: FusedIterator<Item = Item>,
        I9: FusedIterator<Item = Item>,
        I10: FusedIterator<Item = Item>,
        I11: FusedIterator<Item = Item>,
    {
    }
}
