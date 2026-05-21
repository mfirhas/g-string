use g_string::{NoValidation, gstring};

#[test]
fn test_g_string() {
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
}
