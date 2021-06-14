use crate::utils::S;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static LANGUAGE_LOCALE_CODES: Lazy<HashMap<&str, &[&str]>> = Lazy::new(|| {
    vec![
        (S("aa"), &[S("DJ"), S("ER"), S("ET")]),
        (S("af"), &[S("ZA")]),
        (S("ak"), &[S("GH")]),
        (S("am"), &[S("ET")]),
    ]
    .iter()
    .collect::<HashMap<_, _>>()
});

trait BaseProvider {}
