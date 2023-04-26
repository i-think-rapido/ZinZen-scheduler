use std::vec;

use crate::{
    models::{
        goal::{Day, TimeFilter},
        slot::Slot,
        timeline::Timeline,
    },
    services::filter::apply_filter,
    tests::utils::{get_slot, get_timeline_single_slot},
};
use chrono::{Duration, NaiveDate};

#[test]
fn test_after_5am() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let init_duration = Duration::days(1);

    let before_time: Option<usize> = None;
    let after_time: Option<usize> = Some(5);
    let on_days: Option<Vec<Day>> = None;
    let not_on: Option<Vec<Slot>> = None;

    // intiate a sample timeline
    let timeline = get_timeline_single_slot(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let expected_result = Timeline {
        slots: vec![Slot {
            start: NaiveDate::from_ymd_opt(init_year, init_month, init_day)
                .unwrap()
                .and_hms_opt(5, 0, 0)
                .unwrap(),
            end: NaiveDate::from_ymd_opt(init_year, init_month, init_day + 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        }]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = apply_filter(
        &timeline,
        &(TimeFilter {
            before_time,
            after_time,
            on_days,
            not_on,
        }),
    );
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}

#[test]
fn test_normal_workday() {
    let init_year = 2022;
    let init_month = 1;
    let init_day = 1;
    let init_duration = Duration::days(15);

    let before_time: Option<usize> = Some(15);
    let after_time: Option<usize> = Some(5);
    let on_days: Option<Vec<Day>> = Some(vec![Day::Sun, Day::Mon, Day::Tue, Day::Wed, Day::Thu]);
    let not_on: Option<Vec<Slot>> = Some(vec![
        get_slot(Duration::days(1), init_year, init_month, 2, 0, 0),
        get_slot(Duration::days(1), init_year, init_month, 6, 0, 0),
        get_slot(Duration::days(1), init_year, init_month, 11, 0, 0),
    ]);

    // intiate a sample timeline
    let timeline = get_timeline_single_slot(init_duration, init_year, init_month, init_day);
    dbg!(&timeline);

    let expected_result = Timeline {
        slots: vec![
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 3)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 3)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 4)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 4)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 5)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 5)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 9)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 9)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 10)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 10)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 12)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 12)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
            Slot {
                start: NaiveDate::from_ymd_opt(init_year, init_month, 13)
                    .unwrap()
                    .and_hms_opt(5, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(init_year, init_month, 13)
                    .unwrap()
                    .and_hms_opt(15, 0, 0)
                    .unwrap(),
            },
        ]
        .into_iter()
        .collect(),
    };
    dbg!(&expected_result);

    let filtered_timeline = apply_filter(
        &timeline,
        &(TimeFilter {
            before_time,
            after_time,
            on_days,
            not_on,
        }),
    );
    dbg!(&filtered_timeline);

    assert_eq!(filtered_timeline, expected_result);
}
