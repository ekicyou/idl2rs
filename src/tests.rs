use parser::*;


#[test]
fn regex() {
    {
        let test = "12345";
        let (remain, act) = integer(&test).unwrap();
        assert_eq!("12345", act);
        assert_eq!("", remain);
    }
    {
        let test = "0x12ab";
        let (remain, act) = integer(&test).unwrap();
        assert_eq!("0x12ab", act);
        assert_eq!("", remain);
    }
    {
        let test = "1234.5";
        let (remain, act) = float(&test).unwrap();
        assert_eq!("1234.5", act);
        assert_eq!("", remain);
    }
    {
        let test = "a123_t";
        let (remain, act) = identifier(&test).unwrap();
        assert_eq!("a123_t", act);
        assert_eq!("", remain);
    }
}

static TEST_IDL: &'static str = include_str!("ocidl.idl");

#[test]
fn it_works() {
    assert!(true);
}