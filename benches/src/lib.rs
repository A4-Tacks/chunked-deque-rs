//! see benches/

pub fn share_suffix<'a>(s: &[&'a str]) -> &'a str {
    let mut first = *s.first().unwrap();
    for s in s.iter().skip(1) {
        while !first.is_empty() && !s.ends_with(first) {
            first = &first[1..];
        }
    }
    first = first.trim_start_matches('_');
    assert_ne!(first, "");
    first
}

#[rustfmt::skip]
#[macro_export]
macro_rules! bench_caller {
    ($i:ident($d:tt)) => {
        macro_rules! bench {
            ($d($d id:ident),+) => {{
                let share = $crate::share_suffix(&[$d(stringify!($d id)),+]);
                let name = |s: &'static str| s.strip_suffix(share).unwrap().trim_end_matches('_');
                let mut group = $i.benchmark_group(share);
                $d(
                    #[allow(unreachable_code)]
                    let _ = || $d id(loop {});
                    group.bench_function(name(stringify!($d id)), $d id);
                )+
            }};
        }
    };
}
