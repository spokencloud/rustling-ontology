use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    // Days
    // example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 00]), "ahora", "ahora mismo", "en este preciso momento", "en este preciso istante", "inmediatamente");
    example!(v, check_moment!(c, [2013, 2, 12]), "hoy");//, "en este momento");
    example!(v, check_moment!(c, [2013, 2, 11]), "ayer", "el día anterior", "el dia anterior", "el día de antes", "el dia de antes", "la víspera", "la vispera");
    example!(v, check_moment!(c, [2013, 2, 10]), "antes de ayer", "anteayer");
    example!(v, check_moment!(c, [2013, 2, 13]), "mañana", "el día siguiente", "el dia siguiente", "el día de después", "el dia despues", "el día después");
    // TODO: support "el día siguiente al 13 de febrero", "el dia siguiente al 13 de febrero"
    example!(v, check_moment!(c, [2013, 2, 14]), "pasado mañana");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunes", "el lunes", "este lunes");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunes dieciocho febrero", "el lunes dieciocho de febrero");
    example!(v, check_moment!(c, [2013, 2, 19]), "martes", "el martes");
    example!(v, check_moment!(c, [2013, 2, 13]), "miércoles trece febrero", "el miércoles trece de febrero", "miercoles trece de febrero", "el miercoles trece de febrero");
    example!(v, check_moment!(c, [2013, 2, 14]), "jueves", "dos días más tarde", "dos días después", "dos dias mas tarde", "dos dias despues");
    example!(v, check_moment!(c, [2013, 2, 15]), "viernes", "el viernes");
    example!(v, check_moment!(c, [2013, 2, 16]), "sábado", "sabado", "el sábado");
    example!(v, check_moment!(c, [2013, 2, 17]), "domingo", "el domingo");
    example!(v, check_moment!(c, [2013, 3, 1]), "uno de marzo", "el uno de marzo", "viernes uno de marzo", "el viernes uno de marzo");
    example!(v, check_moment!(c, [2013, 3, 1]), "el uno de marzo de dos mil trece", "el uno de marzo del dos mil trece", "el uno de marzo del año dos mil trece");
    example!(v, check_moment!(c, [2013, 3, 2]), "el dos de marzo", "dos de marzo");
    example!(v, check_moment!(c, [2013, 3, 2]), "el dos", "el día dos");
    example!(v, check_moment!(c, [2013, 3, 3]), "el tres de marzo", "el día tres de marzo", "tres marzo");
    example!(v, check_moment!(c, [2013, 4, 5]), "el cinco de abril", "cinco abril", "cinco de abril");
    example!(v, check_moment!(c, [2015, 3, 3]), "el tres de marzo dos mil quince", "el tres de marzo de dos mil quince", "el tres de marzo del dos mil quince", "tres marzo dos mil quince");
    example!(v, check_moment!(c, [2013, 2, 15]), "el quince de febrero", "quince febrero", "quince de febrero");
    example!(v, check_moment!(c, [2013, 2, 16]), "el dieciséis", "el dieciseis");

    // TODO: support/check "17.02", "el 17.02", "el 17-02"
    example!(v, check_moment!(c, [2013, 2, 17]), "diecisiete febrero", "el diecisiete de febrero");
    example!(v, check_moment!(c, [2013, 2, 13]), "miércoles trece", "miercoles trece", "el miércoles trece"); // when today is Tuesday 12, "mercoledì 13" should be tomorrow
    // TODO: support/check "31/10", "el 31/10", "31.10", "el 31.10", "31-10", "el 31-10"
    example!(v, check_moment!(c, [2013, 10, 31]), "treinta y uno octubre", "el treinta y uno de octubre");
    // TODO: support "el día de nochebuena del dos mil catorce"
    //  when today is Tuesday, "lundi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 18]), "el lunes próximo", "el próximo lunes", "el lunes que viene", "el lunes de la semana que viene",  "el lunes de la semana próxima");
    //  when today is Tuesday, "martedì prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 19]), "el martes que viene", "el próximo martes", "el martes próximo", "el martes de la semana que viene", "el martes de la semana próxima", "el martes de la próxima semana");
    example!(v, check_moment!(c, [2013, 2, 13]), "el miércoles que viene", "el miércoles próximo", "el próximo miércoles");
    example!(v, check_moment!(c, [2013, 2, 20]),"el miércoles de la semana que viene", "el miércoles de la semana próxima", "el miércoles de la próxima semana");
    example!(v, check_moment!(c, [2013, 2, 11]), "lunes de esta semana", "el lunes de esta semana");
    example!(v, check_moment!(c, [2013, 2, 12]), "martes de esta semana", "el martes de esta semana");
    example!(v, check_moment!(c, [2013, 2, 13]), "miércoles de esta semana", "el miércoles de esta semana");
    // TODO: support "durante la semana"
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "esta semana");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "la semana pasada", "la pasada semana");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "la próxima semana", "la semana próxima", "la semana que viene");
    example!(v, check_moment!(c, [2013, 1]), "el mes pasado", "el pasado mes");
    example!(v, check_moment!(c, [2013, 3]), "el mes que viene", "el próximo mes", "el mes próximo");
    example!(v, check_moment!(c, [2012]), "el año pasado", "el pasado año");
    example!(v, check_moment!(c, [2013]), "este año");
    example!(v, check_moment!(c, [2014]), "el año que viene", "el próximo año", "el año próximo");
    // TODO: support "el pasado domingo"
    example!(v, check_moment!(c, [2013, 2, 10]), "el domingo pasado", "el domingo de la semana pasada", "el domingo de la pasada semana");
    example!(v, check_moment!(c, [2013, 10, 3]), "el tercer día de octubre");
    // TODO: support "la primera semana del mes de octubre del dos mil catorce"
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "primera semana de octubre de dos mil catorce", "la primera semana de octubre del dos mil catorce");
    example!(v, check_moment!(c, [2013, 10, 7], Grain::Week), "la semana del siete de octubre");
    // fix_example!(v, check_moment!(c, [2015, 10, 31]), "último día de octubre de dos mil quince", "el último día de octubre del dos mil quince");
    // FIXME: resolution issue?
    // fix_example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "última semana de septiembre de dos mil catorce", "la última semana de septiembre de dos mil catorce", "la última semana del mes de septiembre de dos mil catorce", "la última semana de septiembre del año dos mil catorce");


    // Day times
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "a las quince horas", "a las tres de la tarde");
    example!(v, check_moment!(c, [2013, 2, 13, 00]), "medianoche", "las doce de la noche");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "las doce del mediodía", "mediodía", "hoy a mediodía");
    // TODO: support "las doce y cuarto de la mañana"
    example!(v, check_moment!(c, [2013, 2, 12, 12, 15]), "las doce y cuarto", "las doce y cuarto del mediodía");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 55]), "doce menos cinco", "las once y cincuenta y cinco", "las once y cincuenta y cinco minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 30]), "las doce y media", "las doce horas y treinta minutos", "las doce y media del mediodía");
    // TODO: support "las doce y tres de la (media)noche" - resolution issue?
    example!(v, check_moment!(c, [2013, 2, 13, 00, 03]), "las cero horas y tres minutos");
    // FIXME - resolution issue?
    // fix_example!(v, check_moment!(c, [2013, 2, 13, 00, 03]), "hoy a las doce y tres de la noche", "hoy a las doce y tres de la medianoche");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "a las quince y quince", "a las quince horas y quince minutos", "a las tres y cuarto de la tarde");
    // FIXME - confused with morning
    // fix_example!(v, check_moment!(c, [2013, 2, 13, 15, 15]), "a las tres y cuarto mañana por la tarde");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "a las quince y treinta", "a las quince horas y treinta minutos", "a las tres y media de la tarde");
    // FIXME: resolution issue?
    // fix_example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "a las doce menos cuarto del mediodía", "11:45", "a las once horas y cuarenta y cinco minutos", "hoy a las 11:45");

    // Day + day time
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "dos de marzo a las cinco", "dos de marzo a las cinco exactamente", "dos de marzo a las cinco exactas", "dos de marzo a las cinco en punto");
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "dos de marzo a las cinco", "el dos de marzo a las cinco", "el dos de marzo a las cinco de la mañana", "el dos de marzo a las cinco de la madrugada");
    example!(v, check_moment_with_precision!(c, [2013, 3, 2, 5], Precision::Approximate), "dos de marzo sobre las cinco", "el dos de marzo hacia las cinco", "dos de marzo a las cinco más o menos");
    // FIXME: Also time-of-the-day issues, apparently with 'h' - "el dos a las cinco"
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "el dos a las cinco", "el dos a las cinco", "el dos a las cinco de la mañana", "el día dos a las cinco de la madrugada");
    example!(v, check_moment!(c, [2013, 2, 16, 6]), "el dieciséis a las seis de la mañana","el día dieciséis a las seis de la madrugada");
    example!(v, check_moment!(c, [2013, 2, 16, 18]), "el sábado dieciséis a las seis de la tarde");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "mañana a las once");
    example!(v, check_moment!(c, [2013, 2, 14, 11]), "el jueves a las once", "pasado mañana a las once");
    // Warning! 'alle ore 11' and '11:00' don't have same grain);
    // TODO: support "el viernes a las 12h de la mañana"
    example!(v, check_moment!(c, [2013, 2, 15, 12]), "el viernes a las doce del mediodía", "a mediodía el viernes");
    example!(v, check_moment!(c, [2013, 2, 15, 16]), "viernes quince a las dieciséis horas", "el viernes quince a las dieciséis horas", "el viernes quince a las cuatro de la tarde");
    //fix_example!(v, check_moment!(c, [2013, 2, 13, 11, 0]), "miércoles a las 11h", "el miércoles a las 11 de la mañana");

    // In + duration / duration + ago≤
//    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "dentro de un segundo", "en un segundo");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "dentro de un minuto", "en un minuto");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "dentro de dos minutos", "en dos minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "dentro de sesenta minutos", "en sesenta minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "dentro de una hora", "en una hora");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "dentro de veinticuatro horas");
    example!(v, check_moment!(c, [2013, 2, 13]), "dentro de un día", "en un día", "dentro de un dia");
    example!(v, check_moment!(c, [2013, 2, 19]), "dentro de siete dias", "en siete dias");
    example!(v, check_moment!(c, [2013, 2, 19]), "dentro de una semana");
    example!(v, check_moment!(c, [2013, 4, 12]), "dentro de dos meses", "en dos meses");
    example!(v, check_moment!(c, [2014, 2]), "dentro de un año", "en un año");
    example!(v, check_moment!(c, [2013, 2, 12, 2, 30]), "hace dos horas");
    example!(v, check_moment!(c, [2013, 1, 22]), "hace tres semanas");
    example!(v, check_moment!(c, [2012, 11, 12]), "hace tres meses");
    example!(v, check_moment!(c, [2011, 2]), "hace dos años");
    example!(v, check_moment!(c, [2014, 3, 1], Grain::Month), "marzo dos mil catorce", "en marzo dos mil catorce", "por marzo dos mil catorce");
    example!(v, check_moment!(c, [2005, 5, 1], Grain::Month), "mayo dos mil cinco", "en mayo dos mil cinco", "por mayo dos mil cinco");
    // Holidays
    // TODO
    example!(v, check_moment!(c, [2013, 12, 25]), "Navidad");
    // holiday_example!(v, check_moment_span!(c, [2013, 12, 24, 18], [2013, 12, 25, 00]), "la sera di natale", "la notte di Natale");
    // "il primo gennaio & co. works already"
    example!(v, check_moment!(c, [2014, 1, 1]), "año nuevo", "el primero de enero");
    example!(v, check_moment!(c, [2013, 12, 31]), "en nochevieja");
    // holiday_example!(v, check_moment!(c, [2013, 11, 1]), "tutti i santi", "il giorno di tutti i santi", "ognissanti", "il giorno di ognissanti", "il giorno d'ognissanti");
    // "il primo maggio & co. works already"
    // holiday_example!(v, check_moment!(c, [2013, 05, 1]), "festa del lavoro", "la festa dei lavoratori", "il primo maggio");

    // Part of day (morning, afternoon...)
//    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "esta tarde", "por la tarde", "hoy por la tarde");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 15], [2013, 2, 12, 17]), "a media tarde", "en plena tarde", "a plena tarde");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "pronto por la mañana", "a primera hora de la mañana", "por la mañana a primera hora", "a primera hora por la mañana");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "en mitad del día", "a medio día", "a mediodia");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "después de comer", "después de la hora de comer");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "antes de comer");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "durante la comida", "a la hora de comer" );
//    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "después del trabajo", "al salir de trabajar");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 12, 21]), "a primera hora de la tarde", "pronto por la tarde", "por la tarde a primera hora");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 00]), "al final de la tarde", "a última hora de la tarde", "en las últimas horas de la tarde", "por la tarde a última hora");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 6], [2013, 2, 12, 10]), "al inicio del día", "al empezar el día", "a primera hora");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "a mitad del día", "a la mitad del día", "en medio del día", "a medio día", "a mediodia");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "al final del día", "a última hora", "al acabar el día");
//    // TODO: support "esta tarde noche"
//    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "esta noche", "por la noche");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 12, 21]), "a primera hora de la noche", "pronto por la noche", "por la noche pronto");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 00]), "tarde por la noche", "por la noche a última hora", "al final de la velada");
//    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "lunes por la mañana", "el lunes por la mañana");
//    example!(v, check_moment_span!(c, [2013, 2, 18, 12], [2013, 2, 18, 19]), "lunes por la tarde", "el lunes por la tarde");
    // FIXME
    // fix_example!(v, check_moment_span!(c, [2013, 2, 18, 17], [2013, 2, 18, 19]), "el lunes a última hora de la tarde", "a última hora de la tarde el lunes", "por la tarde el lunes a última hora");
    // TODO: support "la mañana del 15 de febrero"
//    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "el quince de febrero por la mañana");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "hoy a las ocho de la tarde", "ocho de la tarde", "las ocho de la noche");
    // TODO: support "las tres de la madrugada"
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "tres de la mañana", "a las tres de la mañana");

    // Part of the week/month
//    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "mañana por la noche", "el miércoles por la noche", "en la noche el miércoles");
    // TODO: support "anoche"
//    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "ayer por la noche");
//    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "este fin de semana", "este fin", "el próximo fin de semana");
//    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 13]), "a principios de semana", "a principios de esta semana");
//    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 15]), "a mitad de semana", "a media semana");
//    example!(v, check_moment_span!(c, [2013, 2, 14], [2013, 2, 18]), "a finales de la semana");
//    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 16]), "durante la semana");
    example!(v, check_moment_span!(c, [2013, 2, 19], [2013, 3, 01]), "a fin de mes", "a finales de mes");
    example!(v, check_moment_span!(c, [2013, 9, 6, 18], [2013, 9, 9, 00]), "el primer fin de semana de septiembre", "el primer finde de septiembre", "el primer fin de semana del mes de septiembre");
    example!(v, check_moment_span!(c, [2013, 9, 13, 18], [2013, 9, 16, 00]), "el segundo fin de semana de septiembre");
    example!(v, check_moment_span!(c, [2013, 9, 27, 18], [2013, 9, 30, 00]), "el último fin de semana de septiembre");

    // Intervals involving cycles
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "dos últimos segundos", "los dos últimos segundos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "los próximos tres segundos", "los tres próximos segundos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "los dos últimos minutos", "últimos dos minutos");
    // FIXME: this is confused b/ time and interval
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "en los próximos tres minutos", "durante los próximos tres minutos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "las tres próximas horas");
    // FIXME: same as above
    // fix_example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "últimos dos días", "los dos pasados días", "durante los dos días pasados");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "próximos tres días", "los próximos tres días");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "últimas dos semanas", "las dos pasadas semanas", "las últimas dos semanas");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "las próximas tres semanas");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "últimos dos meses", "los dos pasados meses");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "los tres próximos meses");
    example!(v, check_moment_span!(c, [2011], [2013]), "los últimos dos años", "los dos pasados años");
    example!(v, check_moment_span!(c, [2014], [2017]), "los próximos tres años");

    // Explicit intervals
    // FIXME: confusion time/date etc. - 2 cases after below do work
    // fix_example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]),  "13 julio - 15 julio",  "del 13 al 15 de julio",  "desde el 13 hasta el 15 de julio",  "del sábado 13 al domingo 15 de julio",  "desde el sábado 13 hasta el domingo 15 de julio",  "del 13 al domingo 15 de julio");
    // FIXME: rustling has a broken rule here with deci* de <monthname>
    // example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "entre las trece y las deciseis de julio");
    // FIXME: same issue as above
    // fix_example!(v, check_moment_span!(c, [2013, 7, 1], [2013, 7, 11]), "del 1 al 10 de julio", "del lunes 1 al miércoles 10 de julio", "dal lunedì primo al mercoledì 10 di luglio");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 18]), "del 13 al 18", "entre el 13 y el 18");
    // FIXME: resolution of span start
    // fix_example!(v, check_moment_span!(c, [2023, 2, 1], [2023, 4, 1]), "entre febrero y marzo de dos mil veintitrés", "entre el mes de febrero y el mes de marzo del dos mil veinttrés");
    example!(v, check_moment_span!(c, [2013, 6, 10], [2013, 7, 2]), "del diez de junio al uno de julio", "entre el diez de junio y el uno de julio", "entre el diez de junio y el primo de julio", "del diez de junio al uno de julio");
    example!(v, check_moment_span!(c, [2017,4,6], [2017,6,9]), "del seis de abril al ocho de junio de dos mil diecisiete", "del seis de abril al ocho de junio del dos mil diecisiete");
    // FIXME
    // fix_example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11]), "09:30 - 11:00 el jueves", "de las 9:30 a las 11 el jueves", "de las 9 y media a las 11 del jueves", "el jueves de las 9h30 a las 11h", "jueves entre las 9 y media y las 11", "jueves de 9h30 a 11h", "el jueves entre las 09:30 y las 11:00");
    // FIXME: resolution issue?
    // fix_example!(v, check_moment_with_direction!(c, [2013, 3, 8], Direction::After), "a partir del 8", "desde el 8 de marzo", "del 8 de marzo en adelante");
    // FIXME: not sure "el jueves después de las 9 y media" is the same as below
    example!(v, check_moment_with_direction!(c, [2013, 2, 14, 9, 30], Direction::After), "a partir de las nueve y treinta del jueves");
    example!(v, check_moment_with_direction!(c, [2013, 11, 1, 16], Direction::After), "después de las cuatro de la tarde el uno de noviembre", "el uno de noviembre después de las dieciséis horas");
    example!(v, check_moment_with_direction!(c, [2013, 11, 1], Direction::After), "después del uno de noviembre");
    // FIXME: OK but resolved as interval
    // fix_example!(v, check_moment_with_direction!(c, [2013, 2, 12, 16], Direction::Before), "antes de las 16h", "hasta las 16:00", "hasta las 4 de la tarde");
    // FIXME: OK but resolution issue
    // fix_example!(v, check_moment_span!(c, [2013, 2, 13, 0], [2013, 2, 13, 6]), "mañana hasta las 6h", "mañana antes de las 6:00", "hasta las 6 mañana");
    // todo_example!(v, check_moment_with_direction!(c, [2013, 2, 20, 10], Direction::After), "el 20 a partir de las 10", "el 20 desde las 10:00", "desde las 10h el 20");
    example!(v, check_moment_with_direction!(c, [2013, 2, 15, 12], Direction::After), "el viernes a partir de mediodía", "el viernes de mediodía en adelante", "desde el viernes a mediodía");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 20], [2013, 2, 20, 18]), "el 20 hasta las 18h");
    // example!(v, check_moment_span!(c, [2014, 9, 14], [2014, 9, 21]), "14-20 sep. dos mil catorce", "14-20 sept. dos mil catorce");
    example!(v, check_moment!(c, [2013, 2, 26]), "dentro de dos semanas", "en dos semanas");
    // 15j != 2 settimanas
    example!(v, check_moment!(c, [2013, 5, 12]), "dentro de tres meses", "en tres meses");
    example!(v, check_moment!(c, [2013, 2, 27]),"en quince días","en los próximos quince días","dentro de quince días");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 7]), "de las cinco a las siete", "entre las cinco y las siete", "de cinco a siete");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "jueves de nueve a once", "el jueves entre las nueve y las once");
//    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "de las doce a las dos del mediodía", "entre las doce del mediodía y las dos de la tarde", "entre las doce y las catorce horas");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "de las once y treinta a la uno y media", "de once y media a una y media");
    // FIXME: rustling doesnt catch this
//    example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "a la uno y media del tarde del sábado veintiuno de septiembre");
    example!(v, check_moment_span!(c, [2013, 3, 25], [2013, 4, 1]), "a finales de marzo", "a finales del mes de marzo");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "a principios de abril", "al comienzo del mes de abril");
    // fix_example!(v, check_moment_span!(c, [2013, 12, 10], [2013, 12, 20]),  "mediados de diciembre", "a mediados de diciembre", "en la mitad del mes de diciembre");
    example!(v, check_moment!(c, [2013, 3]), "marzo", "en marzo", "durante el mes de marzo", "el mes de marzo");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "dentro de un cuarto de hora");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "dentro de media hora");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 15, 0]), "dentro de tres cuartos de hora");
    example!(v, check_moment!(c, [2016, 12, 15]), "el quince de diciembre de dos mil dieciséis");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "durante dos horas", "por dos horas");
    // TODO: support "un día entero", "todo el día"
    example!(v, check_duration!([0, 0, 0, 1]), "durante un día", "durante un dia", "todo un día");
    example!(v, check_duration!([0, 1, 0]), "durante un mes", "por un mes");
    example!(v, check_duration!([1]), "durante un año", "por un año");
    // FIXME: "cerca de hora y media" doesn't work without "una"
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "sobre una hora y media", "una hora y media más o menos", "una hora y media aproximadamente");
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "durante un cuarto de hora más o menos", "durante un cuarto de hora aproximadamente");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "durante una hora", "por una hora");
    example!(v, check_duration!([0, 0, 2]), "durante dos semanas", "por dos semanas");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "un", "uno", "una");
    example!(v, check_integer(11), "once");
    example!(v, check_integer(17), "diecisiete");
    example!(v, check_integer(21), "veintiuno");
    example!(v, check_integer(23), "veintitrés", "veintitres");
    example!(v, check_integer(70), "setenta");
    example!(v, check_integer(78), "setenta y ocho");
    example!(v, check_integer(73), "setenta y tres");
    example!(v, check_integer(80), "ochenta");
    example!(v, check_integer(81), "ochenta y uno");
    example!(v, check_integer(90), "noventa");
    example!(v, check_integer(91), "noventa y uno");
    example!(v, check_integer(99), "noventa y nueve");
    example!(v, check_integer(5000), "cinco mil");
    example!(v, check_integer(200000), "doscientos mil");
    example!(v, check_integer(21011), "veintiuno mil once");
    example!(v, check_integer(721012), "setecientos veintiuno mil doce");
    example!(v, check_integer(31256721), "treinta y un millones doscientos cincuenta y seis mil setecientos veintiuno");
    example!(v, check_integer(33), "treinta y tres");
    example!(v, check_integer(100000), "cien mil");
    example!(v, check_integer(3000000), "tres millones");
    example!(v, check_integer(1200000), "un millón doscientos mil");
    example!(v, check_integer(-1200000), "menos un millón doscientos mil");
    example!(v, check_float(1.1), "uno punto uno", "uno coma uno", "uno punto diez", "uno coma diez");
    example!(v, check_float(0.5), "cero punto cinco", "cero coma cinco", "cero punto cincuenta", "cero coma cincuenta");
    example!(v, check_float(0.3), "cero punto tres", "cero coma tres");
    example!(v, check_float(0.03), "cero punto cero tres", "cero coma cero tres");
    example!(v, check_float(32.75), "treinta y dos punto setenta y cinco", "treinta y dos coma setenta y cinco");
    example!(v, check_float(10.08), "diez punto cero ocho", "diez coma cero ocho");
    // TODO: Check if want/need support for ordinal special character/overscript
    example!(v, check_ordinal(1), "primer", "primero", "primera");
    example!(v, check_ordinal(3), "tercero", "tercera");
    example!(v, check_ordinal(2), "segundo");
    example!(v, check_ordinal(5), "quintos");
}

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "cero coma tres por ciento", "cero coma tres porciento");
    example!(v, check_percentage(15.0), "quince por ciento", "quince porciento");
    example!(v, check_percentage(355.0), "trescientos cincuenta y cinco por ciento");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "tres grados");
    example!(v, check_temperature(32.0, Some("celsius")), "treinta y dos grados celsius", "treinta y dos grados centígrados");
    example!(v, check_temperature(-27.0, Some("degree")), "veintisiete grados bajo cero","menos veintisiete grados");
    example!(v, check_temperature(-27.0, Some("celsius")), "menos veintisiete grados celsius", "menos veintisiete grados centigrados");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "menos cinco grados fahrenheit", "cinco grados fahrenheit bajo cero");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "ciento sesenta y ocho fahrenheit", "ciento sesenta y ocho grados fahrenheit");
    example!(v, check_temperature(10.0, Some("kelvin")), "diez kelvin", "diez grados kelvin");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "ochocientos dólares");
    example!(v, check_finance(10.0, Some("$"), Precision::Approximate), "unos diez dólares", "diez dólares más o menos");
    example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "unos diez dólares americanos");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "tres dólares australianos");
    example!(v, check_finance(3.5, Some("AUD"), Precision::Exact),  "tres dólares australianos y cincuenta centavos");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "cero dólares de hong kong");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "ciento veinticinco dólares canadienses");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact),  "cuarenta y cinco euros");
    example!(v, check_finance(2.05, Some("EUR"), Precision::Exact),  "dos euros y cinco céntimos");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact),  "dos libras");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "veinte libras esterlinas");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "treinta y ocho francos suizos");
    example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "cuatrocientas cuarenta y siete coronas");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "diez mil coronas danesas");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact),  "cien coronas noruegas");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "dos mil cinco coronas suecas");
    example!(v, check_finance(96.0, Some("INR"), Precision::Approximate), "unas noventa y seis rupias");
    // TODO: Support money amounts with cents
    // example!(v, check_finance(5.3, Some("RUB"), Precision::Exact), "cinco rublos y treinta céntimos", "5,3 rublos");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Exact), "exactamente ochenta y nueve yenes japoneses");
    example!(v, check_finance(100.0, Some("CNY"), Precision::Exact), "cien yuanes exactos", "exactamente cien yuanes chinos");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "siete wones surcoreanos");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "tres bitcoins");
    example!(v, check_finance(15.0, Some("$"), Precision::Approximate), "unos quince dólares", "unos quince dolares");
    example!(v, check_finance(3000000.0, Some("EUR"), Precision::Exact), "tres millones de euros");
}
