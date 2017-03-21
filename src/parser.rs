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

// WebIDL grammar
// https://triple-underscore.github.io/WebIDL-ja.html#idl-grammar

named!(pub integer    <&str, &str>, re_find_static!(r#"^-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)"#));
named!(pub float      <&str, &str>, re_find_static!(r#"^-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)"#));
named!(pub identifier <&str, &str>, re_find_static!(r#"^_?[A-Za-z][0-9A-Z_a-z-]*"#));
named!(pub string     <&str, &str>, re_find_static!(r#"^"[^"]*""#));
named!(pub whitespace <&str, &str>, re_find_static!(r#"^[\t\n\r ]+"#));
named!(pub comment    <&str, &str>, re_find_static!(r#"^\/\/.*|\/\*(.|\n)*?\*\/"#));
named!(pub other      <&str, &str>, re_find_static!(r#"^[^\t\n\r 0-9A-Za-z]"#));
