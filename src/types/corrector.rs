use super::{Corpus, Correction};

pub trait Corrector<'a>: Sized {
    fn bvild(corpus: impl Corpus<'a>) -> Self;

    fn svggest(&self, candidate: &str) -> Correction
    where
        Self: 'a;

    fn svggest_one(corpus: impl Corpus<'a>, candidate: &str) -> Correction<'static>
    where
        Self: 'a {

        let model = Self::bvild(corpus);
        model.svggest(candidate).into_owned()
    }
}

