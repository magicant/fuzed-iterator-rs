// Copyright 2023 WATANABE Yuki
// Licensed under the Apache License and the MIT License. Users may choose
// either license (or both), at their option.
//
// ----------------------------------------------------------------------------
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// ----------------------------------------------------------------------------
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![no_std]
#![doc = include_str!("../README.md")]

/// Iterator wrapper that panics if `next` is called after it returns `None`
///
/// See the [crate-level documentation](self) for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Fuze<I> {
    inner: Option<I>,
}

impl<I: Default> Default for Fuze<I> {
    fn default() -> Self {
        Self {
            inner: Some(Default::default()),
        }
    }
}

impl<I> From<I> for Fuze<I> {
    fn from(iter: I) -> Self {
        Fuze { inner: Some(iter) }
    }
}

impl<I> Iterator for Fuze<I>
where
    I: Iterator,
{
    type Item = I::Item;

    /// Returns the next element of the underlying iterator.
    ///
    /// This method drops the underlying iterator once it returns `None`.
    /// If `next` is called after that, it panics.
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self
            .inner
            .as_mut()
            .expect("called `Fuze::next` after it returned `None`");
        let item = inner.next();
        if item.is_none() {
            self.inner = None;
        }
        item
    }

    /// Returns the lower and upper bound on the remaining length of the iterator.
    ///
    /// This method delegates to the underlying iterator's `size_hint` method
    /// if it is available.
    /// After `Fuze::next` returns `None`, it returns `(0, None)`.
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner
            .as_ref()
            .map_or((0, None), |inner| inner.size_hint())
    }
}

impl<I> DoubleEndedIterator for Fuze<I>
where
    I: DoubleEndedIterator,
{
    /// Returns the next element of the underlying iterator.
    ///
    /// This method drops the underlying iterator once it returns `None`.
    /// If `next_back` is called after that, it panics.
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self
            .inner
            .as_mut()
            .expect("called `Fuze::next_back` after it returned `None`");
        let item = inner.next_back();
        if item.is_none() {
            self.inner = None;
        }
        item
    }
}

/// An extension trait that adds the `fuze` method to iterators
pub trait IteratorExt {
    /// Converts `self` into a `Fuze`.
    fn fuze(self) -> Fuze<Self>
    where
        Self: Sized;
}

impl<I: Iterator> IteratorExt for I {
    fn fuze(self) -> Fuze<Self> {
        self.into()
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;

    #[test]
    fn no_panic_until_first_none() {
        let mut i = "foo".chars().fuze();
        assert_eq!(i.next(), Some('f'));
        assert_eq!(i.next(), Some('o'));
        assert_eq!(i.next(), Some('o'));
        assert_eq!(i.next(), None);
    }

    #[test]
    #[should_panic = "called `Fuze::next` after it returned `None`"]
    fn panic_on_another_next_after_none() {
        let mut i = "foo".chars().fuze();
        i.by_ref().for_each(drop);
        i.next();
    }

    #[test]
    fn no_panic_if_fused() {
        let mut i = "foo".chars().fuze().fuse();
        i.by_ref().for_each(drop);
        assert_eq!(i.next(), None);
    }

    #[test]
    fn size_hint() {
        let mut i = [1, 2, 3].iter().fuze();
        assert_eq!(i.size_hint(), (3, Some(3)));
        i.next();
        assert_eq!(i.size_hint(), (2, Some(2)));
        i.next();
        assert_eq!(i.size_hint(), (1, Some(1)));
        i.next();
        assert_eq!(i.size_hint(), (0, Some(0)));
        i.next();
        assert_eq!(i.size_hint(), (0, None));
    }
}

#[cfg(test)]
mod double_ended_iterator_tests {
    use super::*;

    #[test]
    fn no_panic_until_first_none() {
        let mut i = "foo".chars().fuze();
        assert_eq!(i.next_back(), Some('o'));
        assert_eq!(i.next_back(), Some('o'));
        assert_eq!(i.next_back(), Some('f'));
        assert_eq!(i.next_back(), None);
    }

    #[test]
    #[should_panic = "called `Fuze::next_back` after it returned `None`"]
    fn panic_on_another_next_back_after_none() {
        let mut i = "foo".chars().fuze();
        i.by_ref().rev().for_each(drop);
        i.next_back();
    }

    #[test]
    fn no_panic_if_fused() {
        let mut i = "foo".chars().fuze().fuse();
        i.by_ref().rev().for_each(drop);
        assert_eq!(i.next_back(), None);
    }
}
