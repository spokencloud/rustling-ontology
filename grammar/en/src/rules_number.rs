use std::f64;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_3("intersect (with and)",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal("integer (0..19)",
                      b.reg(r#"(none|zilch|naught|nought|nil|zero|one|two|three|fourteen|four|five|sixteen|six|seventeen|seven|eighteen|eight|nineteen|nine|eleven|twelve|thirteen|fifteen|ten)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "none" => 0,
                              "zilch" => 0,
                              "naught" => 0,
                              "nought" => 0,
                              "nil" => 0,
                              "zero" => 0,
                              "one" => 1,
                              "two" => 2,
                              "three" => 3,
                              "four" => 4,
                              "five" => 5,
                              "six" => 6,
                              "seven" => 7,
                              "eight" => 8,
                              "nine" => 9,
                              "ten" => 10,
                              "eleven" => 11,
                              "twelve" => 12,
                              "thirteen" => 13,
                              "fourteen" => 14,
                              "fifteen" => 15,
                              "sixteen" => 16,
                              "seventeen" => 17,
                              "eighteen" => 18,
                              "nineteen" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });
    b.rule_1_terminal("integer (20..90)",
                      b.reg(r#"(twenty|thirty|fou?rty|fifty|sixty|seventy|eighty|ninety)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "twenty" => 20,
                              "thirty" => 30,
                              "fourty" => 40,
                              "forty" => 40,
                              "fifty" => 50,
                              "sixty" => 60,
                              "seventy" => 70,
                              "eighty" => 80,
                              "ninety" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });
    b.rule_2("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             integer_check_by_range!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_2("special composition for missing hundreds like in one twenty two",
             integer_check_by_range!(1, 9),
             integer_check_by_range!(10, 99),
             |a, b| {
                 let value = a.value().value * 100 + b.value().value;
                 IntegerValue::new_with_grain(value, 1)
             });

    b.rule_1_terminal("100, 1_000, 1_000_000, 1_000_000_000",
                      b.reg(r#"(hundred|thousand|million|billion)s?"#)?,
                      |text_match| {
                          let (value, grain) = match text_match.group(1).as_ref() {
                              "hundred" => (100, 2),
                              "thousand" => (1_000, 3),
                              "million" => (1_000_000, 6),
                              "billion" => (1_000_000_000, 9),
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, grain)
                      }
    );

    b.rule_2("200..900, 2_000..9_000, 2_000_000..9_000_000_000",
             integer_check_by_range!(1, 999),
             b.reg(r#"(hundred|thousand|million|billion)s?"#)?,
             |integer, text_match| {
                 let (value, grain) = match text_match.group(1).as_ref() {
                     "hundred" => (100, 2),
                     "thousand" => (1_000, 3),
                     "million" => (1_000_000, 6),
                     "billion" => (1_000_000_000, 9),
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 IntegerValue::new_with_grain(integer.value().value * value, grain)
             }
    );
    b.rule_2("<integer> and a half",
             integer_check!(),
             b.reg(r#"and a half"#)?,
             |integer, _| FloatValue::new(integer.value().value as f64 + 0.5)
    );
    b.rule_2("<integer> and a quarter",
             integer_check!(),
             b.reg(r#"and a quarter"#)?,
             |integer, _| FloatValue::new(integer.value().value as f64 + 0.25)
    );
    b.rule_3("number dot number",
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             b.reg(r#"dot|point"#)?,
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             |a, _, b| {
                 let value: f64 = format!("{}.{}", a.value().value, b.value().value).parse()?;
                 Ok(FloatValue {
                     value,
                     ..FloatValue::default()
                 })
             });
    b.rule_4("number dot zero... number",
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             b.reg(r#"dot|point"#)?,
             b.reg(r#"(?:(?:oh |zero )*(?:oh|zero))"#)?,
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             |a, _, zeros, b| {
                 let zeros_string =  std::iter::repeat("0").take(zeros.group(0).split_whitespace().count()).collect::<String>();
                 let value: f64 = format!("{}.{}{}", a.value().value, zeros_string, b.value().value).parse()?;
                 Ok(FloatValue {
                     value,
                     ..FloatValue::default()
                 })

             });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"minus\s?|negative\s?"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * -1,
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             value: float.value * -1.0,
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             });
    b.rule_2("numbers prefix with +, positive",
             b.reg(r#"plus"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             }
    );
    b.rule_1_terminal("ordinals (first..19th)",
                      b.reg(r#"(zeroth|first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "zeroth" => 0,
                              "first" => 1,
                              "second" => 2,
                              "third" => 3,
                              "fourth" => 4,
                              "fifth" => 5,
                              "sixth" => 6,
                              "seventh" => 7,
                              "eighth" => 8,
                              "ninth" => 9,
                              "tenth" => 10,
                              "eleventh" => 11,
                              "twelfth" => 12,
                              "thirteenth" => 13,
                              "fourteenth" => 14,
                              "fifteenth" => 15,
                              "sixteenth" => 16,
                              "seventeenth" => 17,
                              "eighteenth" => 18,
                              "nineteenth" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_1_terminal("ordinals (20th...90th)",
                      b.reg(r#"(twen|thir|for|fif|six|seven|eigh|nine)tieth"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "twen" => 20,
                              "thir" => 30,
                              "for" => 40,
                              "fif" => 50,
                              "six" => 60,
                              "seven" => 70,
                              "eigh" => 80,
                              "nine" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_2("21th..99th",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             });

    b.rule_3("21th..99th",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, _, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             });

    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
                      b.reg(r#"(hundred|thousand|million|billion)th"#)?,
                      |text_match| {
                          let (value, grain) = match text_match.group(1).as_ref() {
                              "hundred" => (100, 2),
                              "thousand" => (1_000, 3),
                              "million" => (1_000_000, 6),
                              "billion" => (1_000_000_000, 9),
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new_with_grain(value, grain))
                      }
    );

    b.rule_2("ordinal (200..900, 2_000..9_000, 2_000_000..9_000_000_000)",
             integer_check_by_range!(1, 999),
             b.reg(r#"(hundred|thousand|million|billion)th"#)?,
             |integer, text_match| {
                 let (value, grain) = match text_match.group(1).as_ref() {
                     "hundred" => (100, 2),
                     "thousand" => (1_000, 3),
                     "million" => (1_000_000, 6),
                     "billion" => (1_000_000_000, 9),
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(OrdinalValue::new_with_grain(integer.value().value * value, grain))
             }
    );

    b.rule_2("ordinal (1_1_000..9_999_999_000)",
             integer_check_by_range!(1000, 99_999_999_000),
             ordinal_check!(|ordinal: &OrdinalValue| {
            let grain = ordinal.grain.unwrap_or(0);
            grain == 2 || grain % 3 == 0
        }),
             |integer, ordinal| {
                 let grain = ordinal.value().grain.unwrap_or(0);
                 let next_grain = (grain / 3) * 3 + 3;
                 if integer.value().value % 10i64.pow(next_grain as u32) != 0 { return Err(RuleError::Invalid.into()); }
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             }
    );

    b.rule_2("ordinal (101...9_999_999)",
             integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
             ordinal_check_by_range!(1, 99),
             |integer, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             }
    );
    b.rule_3("ordinal (101...9_999_999)",
             integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
             b.reg(r#"and"#)?,
             ordinal_check_by_range!(1, 99),
             |integer, _, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             }
    );
    b.rule_2("the <ordinal>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             |_, ordinal| Ok((*ordinal.value()).prefixed()));
    Ok(())
}
