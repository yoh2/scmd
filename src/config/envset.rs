use crate::util::value::{Referable, SingleOrVec};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type EnvSet = BTreeMap<String, EnvValueWithShorthand>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "op")]
pub enum EnvValue {
    Set {
        value: String,
    },
    Unset,
    Append {
        value: SingleOrVec<String>,
        #[serde(default = "env_value_append_separator_default")]
        separator: String,
    },
}

fn env_value_append_separator_default() -> String {
    " ".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvValueWithShorthand {
    Shorthand(String),
    Full(EnvValue),
}

impl EnvValueWithShorthand {
    pub fn as_env_value(&self) -> Referable<EnvValue> {
        match self {
            EnvValueWithShorthand::Shorthand(s) => {
                Referable::Owned(EnvValue::Set { value: s.clone() })
            }
            EnvValueWithShorthand::Full(e) => Referable::Borrowed(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    macro_rules! test_env_value_serde {
        ($ser_test_name:ident, $de_test_name:ident, $toml_src:expr, $variant:ident { $($field:ident: $fvalue:expr),* $(,)? } $(,)?, ) => {
            test_env_value_serialize!($ser_test_name, $toml_src, $variant { $($field: $fvalue),* } );
            test_env_value_deserialize!($de_test_name, $toml_src, $variant { $($field: $fvalue),* } );
        };
    }

    macro_rules! test_env_value_serialize {
        ($ser_test_name:ident, $toml_src:expr, $variant:ident { $($field:ident: $fvalue:expr),* $(,)? } $(,)? ) => {
            #[test]
            fn $ser_test_name() {
                let env_value = EnvValue::$variant {
                    $($field: $fvalue),*
                };
                let toml = toml::to_string(&env_value).expect("must be serialized");
                assert_eq! (
                    toml,
                    $toml_src,
                );
            }
        };
    }

    macro_rules! test_env_value_deserialize {
        ($de_test_name:ident, $toml_src:expr, $variant:ident { $($field:ident: $fvalue:expr),* $(,)? } $(,)? ) => {
            #[test]
            fn $de_test_name() {
                let parsed = toml::from_str::<EnvValue>($toml_src).expect("must be parsed");
                let EnvValue::$variant { $($field),* } = parsed else {
                    panic!("{:?} must be `{}`", parsed, stringify!($variant));
                };
                $(
                    assert_eq!($field, $fvalue);
                )*
            }
        };
    }

    test_env_value_serde!(
        test_env_value_set_serialize,
        test_env_value_set_deserialize,
        indoc! {r#"
            op = "set"
            value = "foo"
        "#},
        Set {
            value: "foo".to_string(),
        },
    );

    test_env_value_serde!(
        test_env_value_unset_serialize,
        test_env_value_unset_deserialize,
        indoc! {r#"
            op = "unset"
        "#},
        Unset {},
    );

    test_env_value_serialize!(
        test_env_value_append_serialize_short,
        indoc! {r#"
            op = "append"
            value = "foo"
            separator = " "
        "#},
        Append {
            value: SingleOrVec::Single("foo".to_string()),
            separator: " ".to_string(),
        },
    );

    test_env_value_deserialize!(
        test_env_value_append_deserialize_short,
        indoc! {r#"
            op = "append"
            value = "foo"
            # omitting `separator` field implies default value `" "`
        "#},
        Append {
            value: SingleOrVec::Single("foo".to_string()),
            separator: " ".to_string(),
        },
    );

    test_env_value_serde!(
        test_env_value_append_serialize_long,
        test_env_value_append_deserialize_long,
        indoc! {r#"
            op = "append"
            value = ["foo", "bar"]
            separator = ":"
        "#},
        Append {
            value: SingleOrVec::Vec(vec!["foo".to_string(), "bar".to_string()]),
            separator: ":".to_string(),
        },
    );

    #[derive(Debug, Serialize, Deserialize)]
    struct ForShortHandTest {
        foo: EnvValueWithShorthand,
    }

    #[test]
    fn test_env_value_with_shorthand_as_env_value() {
        let shorthand = EnvValueWithShorthand::Shorthand("foo".to_string());
        let EnvValue::Set { value } = &*shorthand.as_env_value() else {
            panic!("{shorthand:?} must be converted as EnvValue::Set");
        };
        assert_eq!(value, "foo");

        let full = EnvValueWithShorthand::Full(EnvValue::Unset);
        let EnvValue::Unset = &*full.as_env_value() else {
            panic!("{full:?} must be converted as EnvValue::Unset");
        };
    }

    #[test]
    fn test_env_value_with_shorthand_serialize_shorthand() {
        let value = ForShortHandTest {
            foo: EnvValueWithShorthand::Shorthand("foo".to_string()),
        };
        let toml = toml::to_string(&value).expect("must be serialized");
        assert_eq!(
            toml,
            indoc! {r#"
                foo = "foo"
            "#}
        );
    }

    #[test]
    fn test_env_value_with_shorthand_serialize_full() {
        let value = ForShortHandTest {
            foo: EnvValueWithShorthand::Full(EnvValue::Set {
                value: "foo".to_string(),
            }),
        };
        let toml = toml::to_string(&value).expect("must be serialized");
        assert_eq!(
            toml,
            indoc! {r#"
                [foo]
                op = "set"
                value = "foo"
            "#},
        );
    }

    #[test]
    fn test_env_value_with_shorthand_deserialize() {
        let parsed = toml::from_str::<ForShortHandTest>(r#"foo = "foo""#).expect("must be parsed");
        let EnvValueWithShorthand::Shorthand(s) = parsed.foo else {
            panic!("{:?} must be `Shorthand`", parsed);
        };
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_env_value_with_shorthand_deserialize2() {
        let parsed = toml::from_str::<ForShortHandTest>(r#"foo = { op = "set", value = "foo" }"#)
            .expect("must be parsed");
        let EnvValueWithShorthand::Full(EnvValue::Set { value }) = parsed.foo else {
            panic!("{:?} must be `Full`", parsed);
        };
        assert_eq!(value, "foo");
    }

    #[test]
    fn test_envset_serialize() {
        let envset = EnvSet::from([
            (
                "SET_VAR1".to_string(),
                EnvValueWithShorthand::Shorthand("foo".to_string()),
            ),
            (
                "SET_VAR2".to_string(),
                EnvValueWithShorthand::Full(EnvValue::Set {
                    value: "foo".to_string(),
                }),
            ),
            (
                "UNSET_VAR".to_string(),
                EnvValueWithShorthand::Full(EnvValue::Unset),
            ),
            (
                "APPEND_VAR1".to_string(),
                EnvValueWithShorthand::Full(EnvValue::Append {
                    value: SingleOrVec::Single("foo".to_string()),
                    separator: " ".to_string(),
                }),
            ),
            (
                "APPEND_VAR2".to_string(),
                EnvValueWithShorthand::Full(EnvValue::Append {
                    value: SingleOrVec::Vec(vec!["foo".to_string(), "bar".to_string()]),
                    separator: ":".to_string(),
                }),
            ),
        ]);
        let toml = toml::to_string(&envset).expect("must be serialized");
        assert_eq!(
            toml,
            // non-table first, and tables are ordered by key dictionary order
            indoc! {r#"
                SET_VAR1 = "foo"
                
                [APPEND_VAR1]
                op = "append"
                value = "foo"
                separator = " "
                
                [APPEND_VAR2]
                op = "append"
                value = ["foo", "bar"]
                separator = ":"
                
                [SET_VAR2]
                op = "set"
                value = "foo"
                
                [UNSET_VAR]
                op = "unset"
            "#},
        );
    }

    macro_rules! assert_env_value_is_shorthand {
        ($envset:expr, $key:expr, $expected:expr) => {
            let value = $envset
                .get($key)
                .expect(concat!("key `", $key, "` must be present"));
            let EnvValueWithShorthand::Shorthand(s) = value else {
                panic!("{:?} must be `Shorthand`", value);
            };
            assert_eq!(s, $expected);
        };
    }

    macro_rules! assert_env_value_is_full {
        ($envset:expr, $key:expr,  $variant:ident { $($field:ident: $fvalue:expr),* $(,)? } $(,)? ) => {
            let value = $envset
                .get($key)
                .expect(concat!("key `", $key, "` must be present"));
            let EnvValueWithShorthand::Full(env_value) = value else {
                panic!("{:?} must be `Shorthand`", value);
            };
            let EnvValue::$variant { $($field,)* } = env_value else {
                panic!("{:?} must be `{}`", env_value, stringify!($variant));
            };
            $(
                assert_eq!($field, &$fvalue);
            )*
        };
    }

    #[test]
    fn test_envset_deserialize() {
        let toml_src = indoc! {r#"
            # set environment variable `SET_VAR1` to `foo`, short description
            SET_VAR1 = 'foo'
            # set environment variable `SET_VAR1` to `foo`, long description
            SET_VAR2 = {op = 'set', value = 'foo'}

            # unset environment variable `UNSET_VAR1`
            UNSET_VAR = {op = 'unset'}

            # append `foo` to environment variable `APPEND_VAR1` with separator ' '
            # e.g. if current APPEND_VAR1 was `abc`, APEND_VER would be `abc foo`
            APPEND_VAR1 = {op = 'append', value = 'foo'}

            # append `foo` and `bar` to environment variable `APPEND_VAR2` with separator ' '
            # e.g. if current APPEND_VAR2 was `abc`, APEND_VER would be `abc foo bar`
            APPEND_VAR2 = {op = 'append', value = ['foo', 'bar']}

            # append `foo` and `bar` to environment variable `APPEND_VAR3` with separator ':'
            # e.g. if current APPEND_VAR3 was `abc`, APEND_VER would be `abc:foo:bar`
            APPEND_VAR3 = {op = 'append', value = ['foo', 'bar'], separator = ':'}
        "#};
        let parsed = toml::from_str::<EnvSet>(toml_src).expect("must be parsed");

        assert_env_value_is_shorthand!(parsed, "SET_VAR1", "foo");
        assert_env_value_is_full!(parsed, "SET_VAR2", Set { value: "foo" });
        assert_env_value_is_full!(parsed, "UNSET_VAR", Unset {});
        assert_env_value_is_full!(
            parsed,
            "APPEND_VAR1",
            Append {
                value: SingleOrVec::Single("foo".to_string()),
                separator: " ".to_string(),
            },
        );
        assert_env_value_is_full!(
            parsed,
            "APPEND_VAR2",
            Append {
                value: SingleOrVec::Vec(vec!["foo".to_string(), "bar".to_string()]),
                separator: " ".to_string(),
            },
        );
        assert_env_value_is_full!(
            parsed,
            "APPEND_VAR3",
            Append {
                value: SingleOrVec::Vec(vec!["foo".to_string(), "bar".to_string()]),
                separator: ":".to_string(),
            },
        );
    }
}
