use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Correction<'a> {
    Correct,
    Incorrect,
    Suggestion(Cow<'a, str>),
}

macro_rules! matches {

    ( $e:expr, $p:pat ) => {
        match $e {
            $p => true,
            _  => false,
        }
    };

}

impl<'a> Correction<'a> {
    pub fn into_owned(self) -> Correction<'static> {
        use Correction::*;
        match self {
            Correct => Correct,
            Incorrect => Incorrect,
            Suggestion(cow) => Suggestion(Cow::Owned(cow.into_owned())),
        }
    }

    pub fn is_correct(&self) -> bool {
        matches!(self, Correction::Correct)
    }

    pub fn is_uncorrectable(&self) -> bool {
        matches!(self, Correction::Incorrect)
    }

    pub fn is_suggestion(&self) -> bool {
        matches!(self, Correction::Suggestion(_))
    }

    pub fn into_option(self) -> Option<Cow<'a, str>> {
        if let Correction::Suggestion(word) = self {
            Some(word)
        } else {
            None
        }
    }

    pub fn as_option(&'a self) -> Option<&'a str> {
        if let Correction::Suggestion(word) = self {
            Some(&word)
        } else {
            None
        }
    }

    pub fn map<'b, F>(self, f: F) -> Correction<'b>
    where
        F: FnOnce(Cow<'a, str>) -> Cow<'b, str> {

        use Correction::*;

        match self {
            Correct => Correct,
            Incorrect => Incorrect,
            Suggestion(word) => Suggestion(f(word)),
        }
    }
}

impl<'a> From<bool> for Correction<'a> {
    fn from(b: bool) -> Self {
        if b {Correction::Correct}
        else {Correction::Incorrect}
    }
}

impl<'a> From<&'a str> for Correction<'a> {
    fn from(s: &'a str) -> Self {
        Correction::Suggestion(Cow::Borrowed(s))
    }
}
