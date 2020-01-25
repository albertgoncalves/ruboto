#![allow(dead_code)]

#[macro_use]
extern crate bencher;

mod sanitize;

use bencher::Bencher;

fn sanitize(b: &mut Bencher) {
    b.iter(|| {
        sanitize::sanitize("&amp;&lt;&gt;\\u201c\\u201d\\u2018\\u2019\n\\*_~`")
    })
}

benchmark_group!(benches, sanitize);
benchmark_main!(benches);
