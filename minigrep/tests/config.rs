extern crate minigrep;

#[test]
fn too_few_arguments() {
    let args = vec![String::from("one"), String::from("too")];
    let config = minigrep::Config::new(&args);
    assert!(config.is_err());
}

#[test]
fn too_many_arguments() {
    let args = vec![String::from("one"), String::from("arg"), String::from("too"), String::from("many")];
    let config = minigrep::Config::new(&args);
    assert!(config.is_err());
}

#[test]
fn just_enough_arguments() {
    let args = vec![String::from("one"), String::from("arg"), String::from("too")];
    let config = minigrep::Config::new(&args);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.path, String::from("too"));
    assert_eq!(config.query, String::from("arg"));
}
