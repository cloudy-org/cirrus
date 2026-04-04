use cirrus_config::v1::{error::Error, template::Template};

#[test]
fn test_valid_template() -> Result<(), Error> {
    let template_string = include_str!("./valid_template.toml");

    let mut template = Template::new(template_string);

    assert!(template.keys.is_none(), "template.keys should be None");

    template.parse_keys()?;

    assert!(template.keys.is_some(), "template.keys should be Some()");

    let template_keys = template.keys.unwrap();

    let meow_key = &template_keys["max_meows"];

    assert_eq!(meow_key.defined_toml_value.as_integer(), Some(15));
    assert_eq!(meow_key.docstring.description.short, None); // TODO: change to some once brief descriptions are implemented

    println!("--> {:?}", meow_key.docstring.description.long.as_ref().unwrap());

    let expected_meow_key_long_description = String::from(
        "Adjust this value to limit the maximum amount of 🐈 cats that are allowed to spawn in.  You can lower this if you're experiencing performance issues.  Do not set this value too high or else a \"🐈 Kitten Overload\" event could occur where the 🐈 kitties multiply beyond your control, steal your processing power and establish a new world order. Kind regards."
    );

    assert_eq!(
        meow_key.docstring.description.long.clone(),
        Some(expected_meow_key_long_description)
    );

    let allow_sentient_ai_key = &template_keys["allow_sentient_ai"];

    assert_eq!(allow_sentient_ai_key.defined_toml_value.as_bool(), Some(true));
    assert_eq!(allow_sentient_ai_key.docstring.description.short, None); // TODO: change to some once brief descriptions are implemented
    assert_eq!(
        allow_sentient_ai_key.docstring.description.long,
        Some(String::from("If you enable this, the AI will eventually kill us all."))
    );

    Ok(())
}