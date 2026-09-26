use std::fmt::{Debug, Formatter};
use std::iter::once;

#[derive(Copy, Clone)]
pub struct TextDictionary(&'static str);

impl Debug for TextDictionary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextDictionary({} bytes)", self.0.len())
    }
}

#[allow(non_upper_case_globals)]
impl TextDictionary {
    pub const STCharacters: Self = Self(include_str!("../data/STCharacters.txt"));
    pub const STPhrases: Self = Self(include_str!("../data/STPhrases.txt"));
    pub const TSCharacters: Self = Self(include_str!("../data/TSCharacters.txt"));
    pub const TSPhrases: Self = Self(include_str!("../data/TSPhrases.txt"));
    pub const TWPhrases: Self = Self(include_str!("../data/TWPhrases.txt"));
    pub const TWPhrasesRev: Self = Self(include_str!("../data/TWPhrasesRev.txt"));
    pub const TWVariants: Self = Self(include_str!("../data/TWVariants.txt"));
    pub const TWVariantsRevPhrases: Self = Self(include_str!("../data/TWVariantsRevPhrases.txt"));
    pub const HKVariants: Self = Self(include_str!("../data/HKVariants.txt"));
    pub const HKVariantsRevPhrases: Self = Self(include_str!("../data/HKVariantsRevPhrases.txt"));
    pub const JPShinjitaiCharacters: Self = Self(include_str!("../data/JPShinjitaiCharacters.txt"));
    pub const JPShinjitaiPhrases: Self = Self(include_str!("../data/JPShinjitaiPhrases.txt"));
    pub const JPVariants: Self = Self(include_str!("../data/JPVariants.txt"));
}

impl TextDictionary {
    pub const fn new(text: &'static str) -> Self {
        Self(text)
    }

    pub const fn as_str(&self) -> &'static str {
        self.0
    }

    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = &'static str> + use<> {
        self.0
            .lines()
            .skip_while(|&line| line.starts_with('#') || line.is_empty())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        self.lines().filter_map(|line| {
            let mut iter = line.split_whitespace();

            Some((iter.next()?, iter.next()?))
        })
    }

    #[inline]
    pub fn iter_inverse(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        self.lines()
            .filter_map(|line| {
                let mut iter = line.split_whitespace();
                let key = iter.next()?;

                Some(once((iter.next()?, key)).chain(iter.map(move |value| (value, key))))
            })
            .flatten()
    }

    #[inline]
    pub fn iter_variants(
        &self,
    ) -> impl Iterator<
        Item = (
            &'static str,
            impl Iterator<Item = &'static str> + Clone + use<>,
        ),
    > + use<> {
        self.lines().filter_map(|line| {
            let mut iter = line.split_whitespace();

            Some((iter.next()?, once(iter.next()?).chain(iter)))
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dictionary {
    STCharacters,
    STPhrases,
    TSCharacters,
    TSPhrases,
    TWPhrases,
    TWPhrasesRev,
    TWVariants,
    TWVariantsRev,
    TWVariantsRevPhrases,
    HKVariants,
    HKVariantsRev,
    HKVariantsRevPhrases,
    JPShinjitaiCharacters,
    JPShinjitaiPhrases,
    JPVariants,
    JPVariantsRev,
}

impl Dictionary {
    /// Returns the text dictionary and whether it is iterated in the inverse direction.
    const fn source(&self) -> (TextDictionary, bool) {
        match self {
            Dictionary::STCharacters => (TextDictionary::STCharacters, false),
            Dictionary::STPhrases => (TextDictionary::STPhrases, false),
            Dictionary::TSCharacters => (TextDictionary::TSCharacters, false),
            Dictionary::TSPhrases => (TextDictionary::TSPhrases, false),
            Dictionary::TWPhrases => (TextDictionary::TWPhrases, false),
            Dictionary::TWPhrasesRev => (TextDictionary::TWPhrasesRev, false),
            Dictionary::TWVariants => (TextDictionary::TWVariants, false),
            Dictionary::TWVariantsRev => (TextDictionary::TWVariants, true),
            Dictionary::TWVariantsRevPhrases => (TextDictionary::TWVariantsRevPhrases, false),
            Dictionary::HKVariants => (TextDictionary::HKVariants, false),
            Dictionary::HKVariantsRev => (TextDictionary::HKVariants, true),
            Dictionary::HKVariantsRevPhrases => (TextDictionary::HKVariantsRevPhrases, false),
            Dictionary::JPShinjitaiCharacters => (TextDictionary::JPShinjitaiCharacters, false),
            Dictionary::JPShinjitaiPhrases => (TextDictionary::JPShinjitaiPhrases, false),
            Dictionary::JPVariants => (TextDictionary::JPVariants, false),
            Dictionary::JPVariantsRev => (TextDictionary::JPVariants, true),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        let (dict, inverse) = self.source();

        if inverse {
            Either::Inverse(dict.iter_inverse())
        } else {
            Either::Forward(dict.iter())
        }
    }
}

/// The iterator returned by [`Dictionary::iter`].
enum Either<F, I> {
    Forward(F),
    Inverse(I),
}

impl<F, I> Iterator for Either<F, I>
where
    F: Iterator,
    I: Iterator<Item = F::Item>,
{
    type Item = F::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Forward(iter) => iter.next(),
            Either::Inverse(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Either::Forward(iter) => iter.size_hint(),
            Either::Inverse(iter) => iter.size_hint(),
        }
    }
}
