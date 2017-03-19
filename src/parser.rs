use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub enum Token<'a> {
    Integer(Cow<'a, str>),
    Float(Cow<'a, str>),
    Identifier(Cow<'a, str>),
    String(Cow<'a, str>),
    WhiteSpace(Cow<'a, str>),
    BlockComment(Cow<'a, str>),
    LineComment(Cow<'a, str>),
    Other(Cow<'a, str>),
}


named!(integer<&str, &str>, re_find_static!(r#"^-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)"#));
