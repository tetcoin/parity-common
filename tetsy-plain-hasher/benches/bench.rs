// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tetsy_plain_hasher::PlainHasher;

fn bench_write_hasher(c: &mut Criterion) {
	c.bench_function("write_plain_hasher", |b| {
		b.iter(|| {
			(0..100u8).fold(PlainHasher::default(), |mut old, new| {
				let bb = black_box([new; 32]);
				old.write(&bb);
				old
			});
		})
	});
	c.bench_function("write_default_hasher", |b| {
		b.iter(|| {
			(0..100u8).fold(DefaultHasher::default(), |mut old, new| {
				let bb = black_box([new; 32]);
				old.write(&bb);
				old
			});
		})
	});
}

criterion_group!(benches, bench_write_hasher);
criterion_main!(benches);
