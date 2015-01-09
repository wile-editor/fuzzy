#![allow(unstable)]
#![feature(plugin)]

#[plugin]
extern crate speculate;
extern crate test;

extern crate fuzzy;

speculate! {
    describe "the real world" {
        context "5286 files from rust repo" {
            before {
                let files = include_str!("../tests/fixtures/rust_repo_files.txt")
                    .lines()
                    .map(|s| s.as_bytes())
                    .collect::<Vec<_>>();

                let go = |&: needle: &[u8]| {
                    for &file in files.iter() {
                        ::test::black_box(::fuzzy::score(file, needle));
                    }
                };

                assert_eq!(files.len(), 5286);
            }

            bench "needle = `ab`" |b| {
                b.iter(|| go(b"ab"));
            }

            bench "needle = `abc`" |b| {
                b.iter(|| go(b"abc"));
            }

            bench "needle = `htn`" |b| {
                b.iter(|| go(b"htn"));
            }

            bench "needle = `src`" |b| {
                b.iter(|| go(b"src"));
            }
        }
    }
}
