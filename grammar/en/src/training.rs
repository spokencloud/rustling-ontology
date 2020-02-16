use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "zero point three per cent");
    example!(v, check_percentage(15.0),"fifteen percent");
    example!(v, check_percentage(202.0), "two hundred two percent");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "three degrees");
    example!(v, check_temperature(32.0, Some("celsius")), "thirty two degrees celsius", "thirty two degrees centigrade");
    example!(v, check_temperature(-27.0, Some("celsius")), "minus twenty seven celsius");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "minus five degrees fahrenheit");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "one hundred and sixty-eight fahrenheit");
    example!(v, check_temperature(10.0, Some("kelvin")), "ten degrees kelvin");
    example!(v, check_temperature(21.0, Some("kelvin")), "twenty one kelvin");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "eight hundred dollars", "eight hundred dollar");
    example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "around ten us dollars");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "exactly three australian dollar");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zero hong kong dollar");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "hundred and twenty five canadian dollars");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "forty five euros");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "two pounds");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "twenty british pounds", "twenty sterlings");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "thirty eight swiss francs");
    // disabled: crowns to general
    // example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "four hundred fourty seven kroner", "four hundred fourty seven crowns");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "ten thousand danish kroner");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "one hundred norwegian crowns", "hundred norwegian kroner");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "two thousand and five swedish crowns");
    example!(v, check_finance(96.0, Some("INR"), Precision::Exact), "ninety six indian rupees");
    example!(v, check_finance(5.0, Some("RUB"), Precision::Exact), "five rubles");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Approximate), "about eighty nine yen");
    example!(v, check_finance(200.0, Some("CNY"), Precision::Exact), "two hundred yuan");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "seven wons");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "three bitcoins");
    example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "two euros and five cents", "two euros five centimes", "two point zero five euros");
    example!(v, check_finance(5.0, Some("cent"), Precision::Exact), "five cents", "five centimes");
    example!(v, check_finance(1.0, Some("cent"), Precision::Exact), "one cent", "one centime");
}


// TODO: Sort out and split by datetime subtype
pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12]), "today");
    example!(v, check_moment!(c, [2013, 2, 11]), "yesterday");
    example!(v, check_moment!(c, [2013, 2, 13]), "tomorrow");
    example!(v, check_moment!(c, [2013, 2, 18]), "monday", "this monday");
    example!(v, check_moment!(c, [2013, 2, 19]), "tuesday", "tuesday the nineteenth");
    example!(v, check_moment!(c, [2013, 2, 14]), "thursday");
    example!(v, check_moment!(c, [2013, 2, 15]), "friday");
    example!(v, check_moment!(c, [2013, 2, 16]), "saturday");
    example!(v, check_moment!(c, [2013, 2, 17]), "sunday");
    example!(v, check_moment!(c, [2013, 3, 1]), "the first of march", "first of march", "march first");
    example!(v, check_moment!(c, [2015, 3, 3]), "march third two thousand fifteen");
    example!(v, check_moment!(c, [2013, 2, 15]), "on the fifteenth");
    example!(v, check_moment!(c, [2013, 2, 15]), "the fifteenth of february", "fifteen of february", "february the fifteenth", "february fifteen", "fifteenth february");
    example!(v, check_moment!(c, [2013, 8, 8]), "august eight");
    example!(v, check_moment!(c, [2014, 10]), "october two thousand fourteen");
    example!(v, check_moment!(c, [2015, 4, 14]), "fourteen april two thousand fifteen", "fourteenth april two thousand fifteen");
    example!(v, check_moment!(c, [2013, 2, 19]), "next tuesday");
    example!(v, check_moment!(c, [2013, 2, 22]), "friday after next");
    example!(v, check_moment!(c, [2013, 3]), "next march");
    example!(v, check_moment!(c, [2014, 3]), "march after next");
    example!(v, check_moment!(c, [2013, 2, 10]), "sunday february the tenth");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "this week", "current week", "coming week");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "last week", "past week", "previous week");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "next week", "the following week");
    example!(v, check_moment!(c, [2013, 1]), "last month");
    example!(v, check_moment!(c, [2013, 3]), "next month");
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "this quarter");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "next quarter");
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "third quarter");
    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "fourth quarter two thousand eighteen");
    example!(v, check_moment!(c, [2012]), "last year");
    example!(v, check_moment!(c, [2013]), "this year", "current year");
    example!(v, check_moment!(c, [2014]), "next year");
    example!(v, check_moment!(c, [2013, 2, 10]), "last sunday");
    example!(v, check_moment!(c, [2013, 2, 5]), "last tuesday");
    example!(v, check_moment!(c, [2013, 2, 13]), "next wednesday");
    example!(v, check_moment!(c, [2013, 2, 14]), "the day after tomorrow");
    example!(v, check_moment!(c, [2013, 2, 14, 17]), "day after tomorrow five pm");
    example!(v, check_moment!(c, [2013, 2, 10]), "the day before yesterday");
    example!(v, check_moment!(c, [2013, 2, 10, 8]), "day before yesterday eight am");
    // FIXME: example!(v, check_moment!(c, [2017, 4, 6]), "in twenty seventeen on thursday the sixth of april");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "at three am", "three in the am", "three oclock am");
    example!(v, check_moment!(c, [2013, 2, 13, 3, 18]), "three eighteen am");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 10]), "ten minutes after eleven", "fifty minutes before noon");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 10]), "exactly ten minutes after eleven", "fifty minutes before noon precisely");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 11, 10], Precision::Approximate), "about fifty minutes before noon");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 15]), "fifteen past six am", "quarter past six am", "quarter past six in the morning");
    example!(v, check_moment!(c, [2013, 2, 10, 6, 15]), "two days ago at fifteen past six am", "the day before yesterday at quarter past six am", "quarter past six in the morning two days ago");
    example!(v, check_moment!(c, [2013, 2, 12, 18, 15]), "fifteen past six pm", "quarter past six pm", "quarter past six in the evening");
    example!(v, check_moment!(c, [2013, 2, 12, 18, 15]), "precisely fifteen past six pm", "quarter past six pm sharp");
    // disabled part of day
    //    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 18, 15], Precision::Approximate), "approximately quarter past six in the evening", "fifteen past six in the afternoon approximately");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "at three pm", "three pm", "three oclock pm", "three o'clock in the afternoon");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 15], Precision::Approximate), "threeish pm", "three pm approximately", "at about three pm");
    example!(v, check_moment!(c, [2013, 2, 12, 14, 50]), "at ten to three", "at ten to three pm", "ten to three in the afternoon");
    example!(v, check_moment!(c, [2013, 2, 13, 3, 15]), "at fifteen past three am", "a quarter past three am", "three fifteen in the morning");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "at fifteen past three pm", "a quarter past three pm");
    example!(v, check_moment!(c, [2013, 2, 12, 18, 45]), "at fifteen to seven pm", "a quarter to seven pm");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 20]), "at twenty past three pm", "three twenty in the afternoon", "twenty after three pm");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "at half past three pm", "half past three pm", "three thirty p.m.", "three thirty p m");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "half three");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "a quarter to noon");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "eight tonight", "eight tonight", "eight this evening");
    // Mixing date and time
    example!(v, check_moment!(c, [2013, 9, 20, 19, 30]), "at seven thirty pm on friday september the twentieth");
    example!(v, check_moment!(c, [2013, 2, 16, 9]), "at nine am on saturday");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "in a sec", "one second from now");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "in a minute", "in one minute");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "in two minutes", "in two more minutes", "two minutes from now");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "in sixty minutes");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "in half an hour");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 0, 0]), "in two and a half hours");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "in one hour");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "in twenty four hours");
    example!(v, check_moment!(c, [2013, 2, 13]), "in a day", "a day from now");
    example!(v, check_moment!(c, [2016, 2]), "three years from today");
    example!(v, check_moment!(c, [2013, 2, 19]), "in seven days");
    example!(v, check_moment!(c, [2013, 2, 19]), "in one week", "in a week");
    example!(v, check_moment!(c, [2013, 2, 5]), "seven days ago");
    example!(v, check_moment!(c, [2013, 2, 5]), "a week ago", "one week ago");
    example!(v, check_moment!(c, [2013, 1, 22]), "three weeks ago");
    example!(v, check_moment!(c, [2012, 11, 12]), "three months ago");
    example!(v, check_moment!(c, [2011, 2]), "two years ago");
    example!(v, check_moment!(c, [2001]), "two thousand one");
    example!(v, check_moment!(c, [2013, 2, 19]), "seven days hence");
    example!(v, check_moment!(c, [2013, 2, 19]), "a week hence", "one week hence");
    example!(v, check_moment!(c, [2013, 3, 5]), "three weeks hence");
    example!(v, check_moment!(c, [2013, 5, 12]), "three months hence");
    example!(v, check_moment!(c, [2015, 2]), "two years hence");
    example!(v, check_moment!(c, [2013, 12]), "one year after christmas");
    example!(v, check_moment!(c, [2014, 3, 1], Grain::Month), "march two thousand fourteen", "in march two thousand fourteen", "for march two thousand fourteen");
    example!(v, check_moment!(c, [2005, 5, 1], Grain::Month), "may two thousand five", "in may two thousand five", "for may two thousand five");
    example!(v, check_moment!(c, [2013, 12, 25]), "christmas", "christmas day");
    example!(v, check_moment!(c, [2013, 12, 31]), "new year's eve", "new years eve");
    example!(v, check_moment!(c, [2014, 1, 1]), "new year's day", "new years day");
    example!(v, check_moment!(c, [2013, 2, 14]), "valentine's day", "valentine day");
    example!(v, check_moment!(c, [2013, 5, 27]), "memorial day");
    example!(v, check_moment!(c, [2013, 5, 12]), "mother's Day");
    example!(v, check_moment!(c, [2013, 6, 16]), "father's Day");
    example!(v, check_moment_span!(c, [2013, 5, 24, 18], [2013, 5, 28, 0]), "memorial day weekend");
    example!(v, check_moment!(c, [2013, 7, 4]), "independence day", "fourth of july");
    example!(v, check_moment!(c, [2013, 9, 2]), "labor day");
    example!(v, check_moment_span!(c, [2013, 8, 30, 18], [2013, 9, 3, 0]), "labor day weekend");
    example!(v, check_moment!(c, [2013, 10, 31]), "halloween");
    example!(v, check_moment!(c, [2013, 11, 28]), "thanksgiving day", "thanksgiving");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "last two seconds");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "next three seconds");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "last two minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "next three minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 3], [2013, 2, 12, 4]), "last one hour");
    example!(v, check_moment_span!(c, [2013, 2, 11, 4], [2013, 2, 12, 4]), "last twenty four hours");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "next three hours");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "last two days", "past two days");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "next three days");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11], Grain::Week), "last two weeks");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11], Grain::Week), "next three weeks");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "last two months");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]),  "next three months");
    example!(v, check_moment_span!(c, [2011], [2013]), "last two years");
    example!(v, check_moment_span!(c, [2014], [2017]), "next three years");
//  strange behavior
//    example!(v, check_moment_span!(c, [2013, 1, 8], [2013, 12, 13]), "january eight to december twelve twenty thirteen");
    example!(v, check_moment_span!(c, [2019, 1, 8], [2019, 12, 13]), "january eight to december twelve two thousand nineteen"); // this helps correct resolution of year in such intervals, for year current and +
    example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11, 0]), "between nine thirty and eleven on thursday");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "eleven thirty to one thirty");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "within two weeks");
    example!(v, check_moment!(c, [2013, 2, 12, 14]), "today at two pm", "at two pm", "two pm");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 14]), "by two pm");
    example!(v, check_moment!(c, [2013, 2, 13, 15, 22]), "three twenty two pm tomorrow");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::After), "after two pm", "from two pm");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::Before), "until two pm", "through two pm");
    example!(v, check_moment_with_direction!(c, [2013, 2, 17], Direction::After), "after five days");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 11], Direction::Before), "before eleven am");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 13, 11]), "tomorrow before eleven am", "thirteenth february two thousand thirteen until eleven am");
    example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "at one thirty pm", "one thirty pm", "at thirteen thirty");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "in fifteen minutes");
    example!(v, check_moment!(c, [2013, 2, 12, 10, 30]), "ten thirty");
    example!(v, check_moment!(c, [2013, 2, 18]), "next monday");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "at twelve pm", "at noon");
    example!(v, check_moment!(c, [2013, 2, 13, 0]), "at twelve am", "at midnight");
    example!(v, check_moment!(c, [2013, 3]), "march", "in march");
    example!(v, check_moment!(c, [2017, 05, 10]), "wednesday the tenth of may");
    example!(v, check_moment!(c, [2013, 2, 12, 9, 9]), "at nine o nine", "at nine o nine am", "at nine o nine in the morning");
    example!(v, check_moment!(c, [2013, 2, 12, 8, 25]), "at eight twenty five", "at eight twenty five am", "at eight twenty five in the morning");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "during two hours");
    example!(v, check_duration!([0, 0, 0, 1], Precision::Approximate), "about one day");
    example!(v, check_duration!([0, 2, 0]), "during two months");
    example!(v, check_duration!([1]), "during a year");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "during one minute and three seconds");
    example!(v, check_duration!([0, 0, 0, 0, 7, 15], Precision::Approximate), "during about seven hours and a quarter");
    example!(v, check_duration!([0, 0, 0, 0, 3, 30], Precision::Approximate), "about three and a half hours", "around three hours and a half");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "about one hour and a half");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "about one hour thirty", "for around one hour and thirty minutes");
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "during about a quarter of an hour"); // , "around a quarter hour");
    example!(v, check_duration!([0, 0, 0, 0, 0, 45]), "for three quarters of an hour");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "during one hour");
    example!(v, check_duration!([0, 0, 2]), "for two weeks");
    example!(v, check_duration!([0, 0, 0, 2], Precision::Approximate), "around two days");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "naught", "nought", "zero", "nil");
    example!(v, check_integer(1), "one");
    example!(v, check_integer(2), "two");
    example!(v, check_integer(33), "thirty three");
    example!(v, check_integer(14), "fourteen");
    example!(v, check_integer(16), "sixteen");
    example!(v, check_integer(17), "seventeen");
    example!(v, check_integer(18), "eighteen");
    example!(v, check_float(1.1), "one point ten", "one point one");
    example!(v, check_float(0.3), "zero point three");
    example!(v, check_float(0.5), "zero point five");
    example!(v, check_float(0.05), "zero point zero five");
    example!(v, check_float(32.75), "thirty-two point seventy-five");
    example!(v, check_float(10.08), "ten point zero eight");
    example!(v,
             check_integer(100000),
             "one hundred thousand");
    example!(v,
             check_integer(3000000),
             "three million");
    example!(v,
             check_integer(1200000),
             "one million two hundred thousands");
    example!(v,
             check_integer(-1200000),
             "negative one million two hundred thousands",
             "minus one million two hundred thousands");
    example!(v, check_integer(5000), "five thousand");
    example!(v, check_integer(122), "one twenty two");
    example!(v, check_integer(200000), "two hundred thousand");
    example!(v, check_integer(21011), "twenty one thousand eleven");
    example!(v,
             check_integer(721012),
             "seven hundred twenty one thousand twelve",
             "seven hundred twenty one thousand and twelve");
    example!(v,
             check_integer(31256721),
             "thirty-one million two hundred fifty six thousand seven hundred twenty one");
    example!(v, check_ordinal(4), "the fourth", "fourth");
    example!(v, check_ordinal(3), "the third", "third");
    example!(v, check_ordinal(2), "the second", "second");
    example!(v, check_ordinal(21), "the twenty first");
}
