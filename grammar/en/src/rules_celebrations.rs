use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};


// TODO: Add locales

pub fn rules_celebration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_1_terminal("christmas",
                      b.reg(r#"christmas(?: day)?"#)?,
                      |_| Ok(helpers::month_day(12, 25)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"christmas(?: day)?(?:'s)? eve"#)?,
                      |_| Ok(helpers::month_day(12, 24)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("new year's eve",
                      b.reg(r#"new year'?s? eve"#)?,
                      |_| Ok(helpers::month_day(12, 31)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("new year's day",
                      b.reg(r#"new year'?s?(?: day)?"#)?,
                      |_| Ok(helpers::month_day(1, 1)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("valentine's day",
                      b.reg(r#"valentine'?s?(?: day)?"#)?,
                      |_| Ok(helpers::month_day(2, 14)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("MLK Day",
                      b.reg(r#"(?:mlk|martin luther king?)(?: junior)? day"#)?,
                      |_| {
                          let third_week_january =
                              helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(1, 1)?)?;
                          let january = helpers::month(1)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          Ok(january.intersect(&third_week_january)?.intersect(&monday)?
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("memorial day",
                      b.reg(r#"memorial day"#)?,
                      |_| {
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          let may = helpers::month(5)?;
                          Ok(monday.last_of(&may)?
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("memorial day weekend",
                      b.reg(r#"memorial day week(?:\s|-)?end"#)?,
                      |_| {
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          let tuesday = helpers::day_of_week(Weekday::Tue)?;
                          let may = helpers::month(5)?;
                          let start = helpers::cycle_nth_after(Grain::Day, -3, &monday.last_of(&may)?)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let end = tuesday.last_of(&may)?
                              .intersect(&helpers::hour(0, false)?)?;
                          Ok(start.span_to(&end, false)?
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("US independence day",
                      b.reg(r#"(independence|national) day"#)?,
                      |_| Ok(helpers::month_day(7, 4)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("labor day",
                      b.reg(r#"labor day"#)?,
                      |_| Ok(helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("flag day",
                      b.reg(r#"flag day"#)?,
                      |_| Ok(helpers::month_day(6, 14)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("patriot day",
                      b.reg(r#"patriot day"#)?,
                      |_| Ok(helpers::month_day(9, 11)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("women's equality day",
                      b.reg(r#"wom[ea]n'?s equality day"#)?,
                      |_| Ok(helpers::month_day(8, 26)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("labor day weekend",
                      b.reg(r#"labor day week(?:\s|-)?end"#)?,
                      |_| {
                          let start = helpers::cycle_nth_after(Grain::Day, -3, &helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let end = helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Tue)?)?
                              .intersect(&helpers::hour(0, false)?)?;
                          Ok(start.span_to(&end, false)?
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("Father's Day",
                      b.reg(r#"father'?s?'? day"#)?,
                      |_| {
                          let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
                          Ok(sundays_of_june.intersect(&second_week_of_june)? // third sunday of June
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("Mother's Day",
                      b.reg(r#"mother'?s? day"#)?,
                      |_| {
                          let sundays_of_may = helpers::month(5)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let first_week_of_may = helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?;
                          Ok(sundays_of_may.intersect(&first_week_of_may)? // second sunday of May
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("halloween day",
                      b.reg(r#"hall?owe?en(?: day)?"#)?,
                      |_| Ok(helpers::month_day(10, 31)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("thanksgiving day",
                      b.reg(r#"thanks?giving(?: day)?"#)?,
                      |_| {
                          let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Thu)?)?;
                          let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
                          Ok(thursday_november.intersect(&fourth_week_of_november)? // fourth thursday of november
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("black friday",
                      b.reg(r#"black friday"#)?,
                      |_| {
                          let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                          let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
                          Ok(thursday_november.intersect(&fourth_week_of_november)? // fourth friday of november
                              .form(Form::Celebration))
                      }
    );

    Ok(())
}