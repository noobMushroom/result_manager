use result_management::routes::result::add_result::Status;

pub struct LKGCompleteResult;

impl LKGCompleteResult {
    pub fn first_term<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(27), None, "WRITT", None),
            ("ENG", Some(5), None, "CN", None),
            ("ENG", Some(8), None, "HW", None),
            ("HIN", Some(29), None, "WRITT", None),
            ("HIN", Some(5), None, "CN", None),
            ("HIN", Some(8), None, "HW", None),
            ("MATHS", Some(20), None, "WRITT", None),
            ("MATHS", Some(5), None, "CN", None),
            ("MATHS", Some(8), None, "HW", None),
            ("ENG_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("A"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("B"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
            ("WORKSHEET", None, Some("A"), "ACTIVITY", None),
        ]
    }

    pub fn half_yearly<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(27), None, "WRITT", None),
            ("ENG", Some(5), None, "CN", None),
            ("ENG", Some(8), None, "HW", None),
            ("HIN", Some(29), None, "WRITT", None),
            ("HIN", Some(5), None, "CN", None),
            ("HIN", Some(8), None, "HW", None),
            ("MATHS", Some(20), None, "WRITT", None),
            ("MATHS", Some(5), None, "CN", None),
            ("MATHS", Some(8), None, "HW", None),
            ("ENG_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("A"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("B"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
            ("WORKSHEET", None, Some("A"), "ACTIVITY", None),
        ]
    }

    pub fn secnod_term<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(23), None, "WRITT", None),
            ("ENG", Some(5), None, "HW", None),
            ("ENG", Some(6), None, "CN", None),
            ("HIN", Some(17), None, "WRITT", None),
            ("HIN", Some(5), None, "HW", None),
            ("HIN", Some(6), None, "CN", None),
            ("MATHS", Some(15), None, "WRITT", None),
            ("MATHS", Some(5), None, "HW", None),
            ("MATHS", Some(5), None, "CN", None),
            ("ENG_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("C"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
        ]
    }

    pub fn annual<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(23), None, "WRITT", None),
            ("ENG", Some(5), None, "HW", None),
            ("ENG", Some(6), None, "CN", None),
            ("HIN", Some(17), None, "WRITT", None),
            ("HIN", Some(5), None, "HW", None),
            ("HIN", Some(6), None, "CN", None),
            ("MATHS", Some(15), None, "WRITT", None),
            ("MATHS", Some(5), None, "HW", None),
            ("MATHS", Some(5), None, "CN", None),
            ("ENG_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("C"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
            ("WORKSHEET", None, Some("A"), "ACTIVITY", None),
        ]
    }
}

pub struct LKGIncompleteResult;

impl LKGIncompleteResult {
    pub fn first_term<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(27), None, "WRITT", None),
            ("ENG", Some(5), None, "CN", None),
            ("ENG", Some(8), None, "HW", None),
            ("HIN", Some(29), None, "WRITT", None),
            ("HIN", Some(5), None, "CN", None),
            ("HIN", Some(8), None, "HW", None),
            ("MATHS", Some(20), None, "WRITT", None),
            ("MATHS", Some(5), None, "CN", None),
            ("MATHS", Some(8), None, "HW", None),
            ("ENG_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("A"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("B"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
            ("WORKSHEET", None, Some("A"), "ACTIVITY", None),
        ]
    }
    pub fn half_yearly<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", None, None, "WRITT", Some(Status::ABSENT)),
            ("ENG", Some(5), None, "CN", None),
            ("ENG", Some(8), None, "HW", None),
            ("HIN", Some(29), None, "WRITT", None),
            ("HIN", Some(5), None, "CN", None),
            ("HIN", Some(8), None, "HW", None),
            ("MATHS", Some(20), None, "WRITT", None),
            ("MATHS", Some(5), None, "CN", None),
            ("MATHS", None, None, "HW", Some(Status::MEDICAL)),
            ("ENG_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("A"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("B"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("B"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("B"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
            ("WORKSHEET", None, Some("A"), "ACTIVITY", None),
        ]
    }

    pub fn secnod_term<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(23), None, "WRITT", None),
            ("ENG", Some(5), None, "HW", None),
            ("ENG", Some(6), None, "CN", None),
            ("HIN", Some(17), None, "WRITT", None),
            ("HIN", Some(5), None, "HW", None),
            ("HIN", Some(6), None, "CN", None),
            ("MATHS", None, None, "WRITT", Some(Status::ABSENT)),
            ("MATHS", Some(5), None, "HW", None),
            ("MATHS", Some(5), None, "CN", None),
            ("ENG_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("C"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
        ]
    }

    pub fn annual<'a>() -> Vec<(
        &'a str,
        Option<i32>,
        Option<&'a str>,
        &'a str,
        Option<Status>,
    )> {
        vec![
            ("ENG", Some(23), None, "WRITT", None),
            ("ENG", None, None, "HW", Some(Status::MEDICAL)),
            ("ENG", Some(6), None, "CN", None),
            ("HIN", Some(17), None, "WRITT", None),
            ("HIN", Some(5), None, "HW", None),
            ("HIN", Some(6), None, "CN", None),
            ("MATHS", Some(15), None, "WRITT", None),
            ("MATHS", Some(5), None, "HW", None),
            ("MATHS", Some(5), None, "CN", None),
            ("ENG_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("ENG_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("HIN_RHYMES", None, Some("C"), "ORAL_PRAC", None),
            ("MATHS_ORAL", None, Some("C"), "ORAL_PRAC", None),
            ("CLASS_PERF", None, Some("C"), "CLASS_PER", None),
            ("WELL_DRESSED", None, Some("B"), "ACTIVITY", None),
            ("DRAW", None, Some("B"), "ACTIVITY", None),
            ("PT", None, Some("B"), "ACTIVITY", None),
            ("WORKSHEET", None, Some("A"), "ACTIVITY", None),
        ]
    }
}
