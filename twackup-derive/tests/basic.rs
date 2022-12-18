use twackup_derive::{StrEnumWithDefault, StrEnumWithError};

#[test]
fn with_default_convert() {
    #[derive(StrEnumWithDefault)]
    enum TestEnum {
        #[twackup(convert = "train")]
        TrainCase,

        #[twackup(convert = "lower")]
        LowerCase,

        #[twackup(convert = "upper")]
        UpperCase,

        #[twackup(convert = "camel")]
        CamelCase,

        #[twackup(convert = "pascal")]
        PascalCase,

        #[twackup(convert = "title")]
        TitleCase,

        #[twackup(convert = "snake")]
        SnakeCase,

        #[twackup(convert = "kebab")]
        KebabCase,

        SingleObject(String),
    }

    assert_eq!(TestEnum::from("inner hello").as_str(), "inner hello");
    assert_eq!(TestEnum::TrainCase.as_str(), "Train-Case");
    assert_eq!(TestEnum::LowerCase.as_str(), "lowercase");
    assert_eq!(TestEnum::UpperCase.as_str(), "UPPERCASE");
    assert_eq!(TestEnum::CamelCase.as_str(), "camelCase");
    assert_eq!(TestEnum::PascalCase.as_str(), "PascalCase");
    assert_eq!(TestEnum::TitleCase.as_str(), "TitleCase");
    assert_eq!(TestEnum::SnakeCase.as_str(), "snake_case");
    assert_eq!(TestEnum::KebabCase.as_str(), "kebab-case");
}

#[test]
fn with_error() {
    #[derive(StrEnumWithError, Debug, PartialEq)]
    enum TestEnum {
        SingleCaseOne,
    }

    assert_eq!(
        TestEnum::try_from("SingleCaseOne"),
        Ok(TestEnum::SingleCaseOne)
    );

    assert!(TestEnum::try_from("SingleCase").is_err());
}

#[test]
fn with_default_convert_all() {
    #[derive(StrEnumWithError)]
    #[twackup(convert_all = "lower")]
    enum TestEnum {
        LowerCase,
        #[twackup(convert = "upper")]
        UpperCase,
        #[twackup(rename = "upper")]
        SingleCase,
    }

    assert_eq!(TestEnum::LowerCase.as_str(), "lowercase");
    assert_eq!(TestEnum::UpperCase.as_str(), "UPPERCASE");
    assert_eq!(TestEnum::SingleCase.as_str(), "upper");
}
