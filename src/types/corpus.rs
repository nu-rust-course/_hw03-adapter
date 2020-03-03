pub trait Corpus<'a>: IntoIterator<Item=&'a str> { }

impl<'a, T: IntoIterator<Item=&'a str>> Corpus<'a> for T { }

