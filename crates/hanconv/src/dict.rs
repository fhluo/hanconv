use RawDictionary::*;
use std::iter::once;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RawDictionary {
    STCharacters,
    STPhrases,
    TSCharacters,
    TSPhrases,
    TWPhrases,
    TWPhrasesRev,
    TWVariants,
    TWVariantsRevPhrases,
    HKVariants,
    HKVariantsRevPhrases,
    JPShinjitaiCharacters,
    JPShinjitaiPhrases,
    JPVariants,
}

impl RawDictionary {
    pub const fn text(&self) -> &'static str {
        match self {
            STCharacters => include_str!("../data/STCharacters.txt"),
            STPhrases => include_str!("../data/STPhrases.txt"),
            TSCharacters => include_str!("../data/TSCharacters.txt"),
            TSPhrases => include_str!("../data/TSPhrases.txt"),
            TWPhrases => include_str!("../data/TWPhrases.txt"),
            TWPhrasesRev => include_str!("../data/TWPhrasesRev.txt"),
            TWVariants => include_str!("../data/TWVariants.txt"),
            TWVariantsRevPhrases => include_str!("../data/TWVariantsRevPhrases.txt"),
            HKVariants => include_str!("../data/HKVariants.txt"),
            HKVariantsRevPhrases => include_str!("../data/HKVariantsRevPhrases.txt"),
            JPShinjitaiCharacters => {
                include_str!("../data/JPShinjitaiCharacters.txt")
            }
            JPShinjitaiPhrases => include_str!("../data/JPShinjitaiPhrases.txt"),
            JPVariants => include_str!("../data/JPVariants.txt"),
        }
    }

    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = &'static str> + use<> {
        self.text()
            .lines()
            .skip_while(|&line| line.starts_with('#') || line.is_empty())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        self.lines().filter_map(|line| {
            let mut iter = line.split_whitespace();

            Some((iter.next()?, iter.next()?))
        })
    }

    pub fn iter_inverse(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        self.lines()
            .filter_map(|line| {
                let mut iter = line.split_whitespace();
                let key = iter.next()?;

                Some(once((iter.next()?, key)).chain(iter.map(move |value| (value, key))))
            })
            .flatten()
    }

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

macro_rules! iter {
    [$a:expr $(,$b:expr)*] => {
        Box::new($a.iter()$(.chain($b.iter()))*)
    };
}

macro_rules! iter_inverse {
    [$a:expr $(,$b:expr)*] => {
        Box::new($a.iter_inverse()$(.chain($b.iter_inverse()))*)
    };
}

impl Dictionary {
    pub fn iter(&self) -> Box<dyn Iterator<Item = (&'static str, &'static str)>> {
        match self {
            Dictionary::STCharacters => iter![STCharacters],
            Dictionary::STPhrases => iter![STPhrases],
            Dictionary::TSCharacters => iter![TSCharacters],
            Dictionary::TSPhrases => iter![TSPhrases],
            Dictionary::TWPhrases => iter![TWPhrases],
            Dictionary::TWPhrasesRev => iter![TWPhrasesRev],
            Dictionary::TWVariants => iter![TWVariants],
            Dictionary::TWVariantsRev => iter_inverse![TWVariants],
            Dictionary::TWVariantsRevPhrases => iter![TWVariantsRevPhrases],
            Dictionary::HKVariants => iter![HKVariants],
            Dictionary::HKVariantsRev => iter_inverse![HKVariants],
            Dictionary::HKVariantsRevPhrases => iter![HKVariantsRevPhrases],
            Dictionary::JPShinjitaiCharacters => iter![JPShinjitaiCharacters],
            Dictionary::JPShinjitaiPhrases => iter![JPShinjitaiPhrases],
            Dictionary::JPVariants => iter![JPVariants],
            Dictionary::JPVariantsRev => iter_inverse![JPVariants],
        }
    }
}
