#[macro_export]
macro_rules! from_str_from_css {
    ($t:ty) => {};
}

#[macro_export]
macro_rules! impl_parse_for_keyword_enum {
    ($e:ident, $($rest:tt)*) => {};
}

#[allow(unused)]
pub type CssParseResult<'i, T> = Result<T, ()>;

#[allow(unused)]
pub trait FromCss: Sized {
    fn from_css(_css: &str) -> Result<Self, ()> {
        Err(())
    }
}

#[allow(unused)]
pub struct Parser<'i, 't> {
    _marker: std::marker::PhantomData<(&'i (), &'t ())>,
}

#[allow(unused)]
impl<'i, 't> Parser<'i, 't> {
    pub fn next(&mut self) -> Result<Token<'i>, ()> {
        Err(())
    }
    pub fn new_unexpected_token_error(&self, _token: Token<'i>) {
    }
    pub fn parse_entirely<F, T, E>(&mut self, _f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
    {
        unimplemented!()
    }
}

#[allow(unused)]
pub enum Token<'i> {
    Percentage { unit_value: f32 },
    Dimension { unit: std::borrow::Cow<'i, str>, value: f32, has_sign: bool },
    Ident(std::borrow::Cow<'i, str>),
}

#[allow(unused)]
pub fn parse_css_str_entirely<T: FromCss>(_: &str) -> Result<T, ()> {
    Err(())
}

#[derive(Debug)]
pub struct ParseError;

pub type ParseResult<T> = Result<T, ParseError>;
