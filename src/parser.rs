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

named!(integer    <&str, &str>, re_find_static!(r#"^-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)"#));
named!(float      <&str, &str>, re_find_static!(r#"^-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)"#));
named!(identifier <&str, &str>, re_find_static!(r#"^_?[A-Za-z][0-9A-Z_a-z-]*"#));
named!(string     <&str, &str>, re_find_static!(r#"^"[^"]*""#));
named!(whitespace <&str, &str>, re_find_static!(r#"^[\t\n\r ]+"#));
named!(comment    <&str, &str>, re_find_static!(r#"^\/\/.*|\/\*(.|\n)*?\*\/"#));
named!(other      <&str, &str>, re_find_static!(r#"^[^\t\n\r 0-9A-Za-z]"#));
