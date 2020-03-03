use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Correction<'a> {
    Correct,
    Incorrect,
    Suggestion(Cow<'a, str>),
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
