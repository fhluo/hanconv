use RawDictionary::*;

pub enum RawDictionary {
    STCharacters,
    STPhrases,
    TSCharacters,
    TSPhrases,
    TWPhrasesIT,
    TWPhrasesName,
    TWPhrasesOther,
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
            TWPhrasesIT => include_str!("../data/TWPhrasesIT.txt"),
            TWPhrasesName => include_str!("../data/TWPhrasesName.txt"),
            TWPhrasesOther => include_str!("../data/TWPhrasesOther.txt"),
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

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        self.text().lines().filter_map(|line| {
            let mut iter = line.split_whitespace();

            if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
                Some((key, value))
            } else {
                None
            }
        })
    }

    pub fn inv_iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + use<> {
        self.text()
            .lines()
            .filter_map(|line| {
                let mut iter = line.split_whitespace().peekable();

                if let (Some(key), Some(_)) = (iter.next(), iter.peek()) {
                    Some(iter.map(|value| (value, key)).collect::<Vec<_>>())
                } else {
                    None
                }
            })
            .flatten()
    }

    pub fn var_iter(&self) -> impl Iterator<Item = (&'static str, Vec<&'static str>)> + use<> {
        self.text().lines().filter_map(|line| {
            let mut iter = line.split_whitespace().peekable();

            if let (Some(key), Some(_)) = (iter.next(), iter.peek()) {
                Some((key, iter.collect::<Vec<_>>()))
            } else {
                None
            }
        })
    }
}

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

macro_rules! inv_iter {
    [$a:expr $(,$b:expr)*] => {
        Box::new($a.inv_iter()$(.chain($b.inv_iter()))*)
    };
}

impl Dictionary {
    pub fn iter(&self) -> Box<dyn Iterator<Item = (&'static str, &'static str)>> {
        match self {
            Dictionary::STCharacters => iter![STCharacters],
            Dictionary::STPhrases => iter![STPhrases],
            Dictionary::TSCharacters => iter![TSCharacters],
            Dictionary::TSPhrases => iter![TSPhrases],
            Dictionary::TWPhrases => iter![TWPhrasesIT, TWPhrasesName, TWPhrasesOther],
            Dictionary::TWPhrasesRev => inv_iter![TWPhrasesIT, TWPhrasesName, TWPhrasesOther],
            Dictionary::TWVariants => iter![TWVariants],
            Dictionary::TWVariantsRev => inv_iter![TWVariants],
            Dictionary::TWVariantsRevPhrases => iter![TWVariantsRevPhrases],
            Dictionary::HKVariants => iter![HKVariants],
            Dictionary::HKVariantsRev => inv_iter![HKVariants],
            Dictionary::HKVariantsRevPhrases => iter![HKVariantsRevPhrases],
            Dictionary::JPShinjitaiCharacters => iter![JPShinjitaiCharacters],
            Dictionary::JPShinjitaiPhrases => iter![JPShinjitaiPhrases],
            Dictionary::JPVariants => iter![JPVariants],
            Dictionary::JPVariantsRev => inv_iter![JPVariants],
        }
    }
}
