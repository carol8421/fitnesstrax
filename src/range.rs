extern crate chrono;
extern crate chrono_tz;
extern crate dimensioned;
extern crate gio;
extern crate gtk;

use std::collections::HashMap;

use emseries::{Recordable, UniqueId};
pub use fitnesstrax_lib::{Params, Result, Trax, TraxRecord};

#[derive(Clone, Debug)]
pub struct Range<A> {
    pub start: A,
    pub end: A,
}

impl<A> Range<A> {
    pub fn new(start: A, end: A) -> Range<A> {
        Range { start, end }
    }

    /*
    pub fn map<F, B>(&self, f: F) -> Range<B>
    where
        F: Fn(&A) -> B,
    {
        Range {
            start: f(&self.start),
            end: f(&self.end),
        }
    }
    */
}

pub fn dates_in_range(
    range: &Range<chrono::Date<chrono_tz::Tz>>,
) -> Vec<chrono::Date<chrono_tz::Tz>> {
    let mut dates = vec![];
    let mut current = range.start.clone();
    while current <= range.end {
        dates.push(current.clone());
        current = current.succ();
    }

    dates
}

pub fn group_by_date(
    range: &Range<chrono::Date<chrono_tz::Tz>>,
    records: Vec<(UniqueId, TraxRecord)>,
) -> HashMap<chrono::Date<chrono_tz::Tz>, Vec<(UniqueId, TraxRecord)>> {
    let mut groups: HashMap<chrono::Date<chrono_tz::Tz>, Vec<(UniqueId, TraxRecord)>> =
        HashMap::new();
    for date in dates_in_range(range) {
        let compare_against_date = move |target_date| target_date == date;

        let recs = records
            .iter()
            .filter(|r| compare_against_date(r.1.timestamp().0.date()))
            .cloned()
            .collect();

        groups.insert(date, recs);
    }

    groups
}

#[cfg(test)]
mod test {
    use super::{dates_in_range, group_by_date, Range};
    use chrono::TimeZone;
    use chrono_tz::America::New_York;
    use dimensioned::si::KG;
    use emseries::{DateTimeTz, UniqueId};
    use fitnesstrax::TraxRecord;

    #[test]
    fn it_creates_a_list_of_dates() {
        let range = Range::new(New_York.ymd(2019, 5, 1), New_York.ymd(2019, 5, 15));
        let h = dates_in_range(&range);

        assert_eq!(h.len(), 15);
        assert_eq!(h[0], New_York.ymd(2019, 5, 1));
        assert_eq!(h[5], New_York.ymd(2019, 5, 6));
        assert_eq!(h[14], New_York.ymd(2019, 5, 15));
    }

    #[test]
    fn it_correctly_groups_items_by_date() {
        let range = Range::new(New_York.ymd(2019, 5, 1), New_York.ymd(2019, 5, 15));

        let recs = vec![
            (
                UniqueId::new(),
                TraxRecord::weight(
                    DateTimeTz(New_York.ymd(2019, 5, 5).and_hms(0, 0, 0)),
                    50. * KG,
                ),
            ),
            (
                UniqueId::new(),
                TraxRecord::weight(
                    DateTimeTz(New_York.ymd(2019, 5, 5).and_hms(0, 0, 0)),
                    57. * KG,
                ),
            ),
            (
                UniqueId::new(),
                TraxRecord::weight(
                    DateTimeTz(New_York.ymd(2019, 5, 7).and_hms(0, 0, 0)),
                    58. * KG,
                ),
            ),
        ];

        let groups = group_by_date(&range, recs);

        assert_eq!(
            groups.get(&New_York.ymd(2019, 5, 5)).map(|v| v.len()),
            Some(2)
        );
        assert_eq!(
            groups.get(&New_York.ymd(2019, 5, 7)).map(|v| v.len()),
            Some(1)
        );
        assert_eq!(
            groups.get(&New_York.ymd(2019, 5, 8)).map(|v| v.len()),
            Some(0)
        );
    }

    /*
    #[test]
    fn it_shows_various_dates_and_times() {
        let today = chrono::Utc::today();
        println!("{}", today);
        println!("{}", today.with_timezone(&chrono_tz::America::Los_Angeles));
        println!("{}", today.with_timezone(&chrono_tz::America::New_York));
        println!("{}", today.with_timezone(&chrono_tz::Australia::Melbourne));

        assert_eq!(false, true);
    }
    */
}
