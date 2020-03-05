#[macro_export]
macro_rules! convert_correction {
    (<$lv:lifetime> $from:ty, $correct:ident, $incorrect:ident, $suggestion:ident) => {
        impl<$lv> From<$from> for m::Correction<$lv> {
            fn from(c: $from) -> Self {
                use m::Correction as To;
                type From<$lv> = $from;
                match c {
                    From::$correct => To::Correct,
                    From::$incorrect => To::Incorrect,
                    From::$suggestion(s) => To::Suggestion(s.into()),
                }
            }
        }
    };

    (<$lv:lifetime> $from:ty) => {
        m::convert_correction!(<$lv> $from, Correct, Incorrect, Suggestion);
    };

    ($from:ty) => {
        m::convert_correction!(<'a> $from);
    };
}

#[macro_export]
macro_rules! adapt_corrector {
    (<$lv:lifetime> $from:ty, new: $new:ident, learn: $learn:ident, suggest: $suggest:ident) => {
        impl<$lv> m::Corrector<$lv> for $from {
            fn bvild(corpus: impl m::Corpus<$lv>) -> Self {
                let mut result = Self::$new();
                for word in corpus {
                    result.$learn(word.into());
                }
                result
            }

            fn svggest(&self, candidate: &str) -> m::Correction {
                m::Correction::from(self.$suggest(candidate))
            }
        }
    };

    (<$lv:lifetime> $from:ty, build: $build:ident, suggest: $suggest:ident) => {
        impl<$lv> m::Corrector<$lv> for $from {
            fn bvild(corpus: impl m::Corpus<$lv>) -> Self {
                Self::$build(corpus)
            }

            fn svggest(&self, candidate: &str) -> m::Correction {
                m::Correction::from(self.$suggest(candidate))
            }
        }
    };

    (<$lv:lifetime> $from:ty, new: $new:ident, learn: $learn:ident) => {
        m::adapt_corrector!(<$lv> $from, new: $new, learn: $learn, suggest: suggest);
    };

    (<$lv:lifetime> $from:ty, build: $build:ident) => {
        m::adapt_corrector!(<$lv> $from, build: $build, suggest: suggest);
    };

    (<$lv:lifetime> $from:ty, new) => {
        m::adapt_corrector!(<$lv> $from, new: new, learn: learn, suggest: suggest);
    };

    (<$lv:lifetime> $from:ty, build) => {
        m::adapt_corrector!(<$lv> $from, build: build, suggest: suggest);
    };

    (<$lv:lifetime> $from:ty) => {
        m::adapt_corrector!(<$lv> $from, new);
    };

    ($from:ty, $($rest:tt)*) => {
        m::adapt_corrector!(<'a> $from, $($rest)*);
    };

    ($from:ty) => {
        m::adapt_corrector!(<'a> $from);
    };
}

#[macro_export]
macro_rules! test_svggest_one {
    ($t:ty, $name:ident, $corpus:tt, $cand:expr, $goal:expr) =>
    {
        #[test]
        fn $name() {
            assert_eq!( <$t as m::Corrector>::svggest_one(vec! $corpus, $cand),
                        m::Correction::from($goal) );
        }
    };

}

#[macro_export]
macro_rules! adapt {
    {
        [$($correction_toks:tt)*]
        [$($corrector_toks:tt)*]
        $($extra:item)*
    } => {
        #[allow(missing_docs)]
        mod adaptations {
            use super::*;
            use ::hw03_adapter as m;
            $($extra)*
            m::convert_correction!( $($correction_toks)* );
            m::adapt_corrector!( $($corrector_toks)* );
        }
    };
}
