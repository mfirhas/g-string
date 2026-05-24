use g_string::{NoValidation, gformat, gstring};

#[test]
fn test_g_string() {
    // let a = gstring!("anu: {}, {}", some_text, another, ValidationType, ...)
    let _ret = gstring!("wd", NoValidation, 2, 3).validate().unwrap();
    dbg!(&_ret);
    println!("{}", _ret);
    let _ret = gstring!(
        r#"lsmdf
        dfdf"#
    );
    dbg!(&_ret);
    println!("{}", _ret);

    let ret = gstring!("🚀");
    dbg!(&ret);
    println!("{ret}");

    // wont compile
    // let ret = gstring!("🚀", NoValidation, 0, 100, true);
    // dbg!(&ret);
    // println!("{ret}");
    //

    let s = String::from("a");
    // let b = s.find(pat)
    //

    let a = gformat!("sdf: {}, {}", "123", 44).unwrap();
    println!("{a}");
}
