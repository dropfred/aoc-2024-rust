use std::iter::Peekable;

pub struct Sep<I, S>
where
    I: Iterator,
    I::Item: From<S>,
    S: Clone
{
    iter: Peekable<I>,
    separator: S,
    insert: bool
}

impl<I, S> Sep<I, S>
where
    I: Iterator,
    I::Item: From<S>,
    S: Clone
{
    fn new(iter: I, separator: S) -> Self {
        Self {
            iter: iter.peekable(),
            separator,
            insert: false
        }
    }
}

impl<I, S> Iterator for Sep<I, S>
where
    I: Iterator,
    I::Item: From<S>,
    S: Clone
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.insert {
            self.insert = false;
            Some(Self::Item::from(self.separator.clone()))
        } else {
            let n = self.iter.next();
            self.insert = self.iter.peek().is_some();
            n
        }
    }
}

pub trait SepIterator<S>: Iterator {
    fn sep(self, separator: S) -> Sep<Self, S>
    where
        Self: Sized,
        Self::Item: From<S>,
        S: Clone
    {
        Sep::new(self, separator)
    }
}

impl<I, S> SepIterator<S> for I where I: Iterator, I::Item: From<S>, S: Clone {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sep() {
        let s: String = "".chars().sep(' ').collect();
        assert_eq!(s, "");

        let s: String = "a".chars().sep(' ').collect();
        assert_eq!(s, "a");

        let s: String = "ab".chars().sep(' ').collect();
        assert_eq!(s, "a b");

        let s: String = "ab".chars().map(|c| c.to_string()).sep(", ").collect();
        assert_eq!(s, "a, b");
    }
}