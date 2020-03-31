use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"sec(?:ond)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );

    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"min(?:ute)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"hours?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"(?:buss?iness? |worki?n?g? ?|week ?)?days?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"weeks?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"months?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("quarter (unit-of-duration)",
                      b.reg(r#"quarters?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Quarter))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"years?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("quarter of an hour",
                      b.reg(r#"(?:a )?quarter (?:of an )hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"half an? hour|an? half hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"three quarters of an hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> more <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"more"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2("<integer> and a half hours",
             integer_check_by_range!(0),
             b.reg(r#"and (?:an? )?half hours?"#)?,
             |integer, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"and (?:an? )?half"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and a quarter",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"and (?:a? )?quarter"#)?,
             |integer, uod, _| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> and a half <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"and (?:an? )?half"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> and a quarter <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"and (?:a? )?quarter"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"hours?"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_2("a <unit-of-duration>",
             b.reg(r#"an?"#)?,
             unit_of_duration_check!(),
             |_, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 1).into()))
    );
    b.rule_2("for <duration>",
             b.reg(r#"for"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("during <duration>",
             b.reg(r#"during"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("after <duration>",
             b.reg(r#"after"#)?,
             duration_check!(),
             |_, duration| Ok(duration
                 .value()
                 .in_present()?
                 .mark_after_start())
    );
    b.rule_3("<duration> and <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"and"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );

    b.rule_2("about <duration>",
             b.reg(r#"(?:about|around|approximately|roughly)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("<duration> approximately",
             duration_check!(),
             b.reg(r#"(?:about|around|approximately|roughly)"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("exactly <duration>",
             b.reg(r#"exactly|precisely"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );

    b.rule_2("<duration> exactly",
             duration_check!(),
             b.reg(r#"exactly|precisely"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    Ok(())
}