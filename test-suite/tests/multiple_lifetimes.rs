use hard_xml::{XmlRead, XmlResult, XmlWrite};
use std::borrow::Cow;

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "foo")]
struct Foo<'a, 'b, 'c> {
    #[xml(attr = "bar")]
    bar: Cow<'a, str>,
    #[xml(attr = "baz")]
    baz: Cow<'b, str>,
    #[xml(attr = "quz")]
    quz: Cow<'c, str>,
}

#[test]
fn test() -> XmlResult<()> {
    let _ = env_logger::builder()
        .is_test(true)
        .format_timestamp(None)
        .try_init();

    assert_eq!(
        (Foo {
            bar: "bar".into(),
            baz: "baz".into(),
            quz: "quz".into(),
        })
        .to_string()?,
        r#"<foo bar="bar" baz="baz" quz="quz"/>"#
    );

    assert_eq!(
        Foo::from_str(r#"<foo bar="bar" baz="baz" quz="quz"/>"#)?,
        Foo {
            bar: "bar".into(),
            baz: "baz".into(),
            quz: "quz".into(),
        }
    );

    Ok(())
}
