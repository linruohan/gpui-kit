//! 日期类组件共用的辅助函数。

use jiff::civil;

/// 返回指定日期对应的节假日文案，格式为 `"{日} {节日名} {休|班}"`。
///
/// 优先使用法定节假日（含调班标记）；否则依次匹配节气、农历节日、
/// 公历节日，最后回退到农历日名，并在周末追加 `休`。
pub fn get_holiday_by_tyme4rs(date: civil::Date) -> String {
    use tyme4rs::tyme::{
        Culture, holiday::LEGAL_HOLIDAY_NAMES, lunar::LUNAR_DAY_NAMES, solar::SolarDay,
    };

    let solar_day = SolarDay::from_ymd(
        date.year() as isize,
        date.month() as usize,
        date.day() as usize,
    );
    let week = solar_day.get_week().get_index();
    if let Some(holiday) = solar_day.get_legal_holiday() {
        format!(
            "{} {} {}",
            date.day(),
            LEGAL_HOLIDAY_NAMES[holiday.get_index()],
            if holiday.is_work() { "班" } else { "休" }
        )
    } else {
        let lunar = solar_day.get_lunar_day();
        // 农历传统节日
        let lunar_festival = lunar.get_festival();
        // 阳历现代节日
        let solar_festival = solar_day.get_festival();
        let lunar_day_name = LUNAR_DAY_NAMES[lunar.get_day() - 1].to_string();
        let jieqi = solar_day.get_term_day();
        let jieqi_index = jieqi.get_day_index();

        format!(
            "{} {} {}",
            date.day(),
            if jieqi_index == 0 {
                jieqi.get_name()
            } else if let Some(lu_fe) = lunar_festival {
                lu_fe.get_name()
            } else if let Some(so_fe) = solar_festival {
                so_fe.get_name()
            } else {
                lunar_day_name
            },
            if week == 0 || week == 6 { "休" } else { "" }
        )
    }
}
