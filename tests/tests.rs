#![allow(staged_experimental)]
#![feature(plugin)]

#[plugin]
extern crate speculate;

extern crate fuzzy;

macro_rules! go {
    ($haystacks:expr, $($needle:expr => $expected:expr),+) => {{
        $(
            let haystacks = &$haystacks;
            let expected = $expected;
            let got = ::fuzzy::sorted(haystacks, $needle);
            let got = &got[..expected.len()];
            if expected != got {
                let expected = expected.iter()
                    .map(|&b| String::from_utf8_lossy(b).into_owned())
                    .collect::<Vec<_>>();
                let got = got.iter()
                    .map(|&b| String::from_utf8_lossy(b).into_owned())
                    .collect::<Vec<_>>();

                assert_eq!(expected, got);
            }
        )+
    }}
}

speculate! {
    describe "score" {
        context "empty needle" {
            it "returns 0" {
                assert_eq!(0, ::fuzzy::score(b"", b""));
            }
        }

        context "needle not a subsequence of haystack" {
            it "returns 0" {
                assert_eq!(0, ::fuzzy::score(b"abc", b"xyz"));
            }
        }

        it "..." {
            assert_eq!(4, ::fuzzy::score(b"aaa", b"aa"));
            assert_eq!(3, ::fuzzy::score(b"abc", b"a"));
            assert_eq!(4, ::fuzzy::score(b"xyza", b"xa"));
        }

        it "is case-insensitive" {
            assert!(::fuzzy::score(b"foo", b"F") > 0);
            assert!(::fuzzy::score(b"Foo", b"f") > 0);
        }

        context "bonus" {
            context "is given to" {
                it "first char match" {
                    go! {
                        [b"barfoo", b"foobar"],
                        b"b" => [b"barfoo", b"foobar"],
                        b"f" => [b"foobar", b"barfoo"]
                    };
                }

                it "uppercase char match" {
                    go! {
                        [b"foob", b"fOoBa", b"fooBAR"],
                        b"o" => [b"fOoBa"],
                        b"ob" => [b"fOoBa", b"fooBAR", b"foob"],
                        b"ba" => [b"fooBAR", b"fOoBa"]
                    };
                }

                it "matches just after `-`, `.`, `/`, `:`, and `_`" {
                    go! {
                        [b"foo", b"f:oo", b"bar", b"b/a-r", b"ba_z", b"b.az"],
                        b"f" => [b"foo", b"f:oo"],
                        b"o" => [b"f:oo", b"foo"],
                        b"a" => [b"b.az", b"b/a-r", b"bar", b"ba_z"],
                        b"r" => [b"b/a-r", b"bar"],
                        b"z" => [b"ba_z", b"b.az"]
                    };
                }
            }
        }
    }

    describe "sorted" {
        it "works!" {
            assert_eq!([b"foo", b"bar"],
                       ::fuzzy::sorted(&[b"bar", b"foo"], b"f"));
        }

        context "score is same" {
            it "ranks shorter haystack first" {
                go! {
                    [b"foo", b"foobar", b"baz", b"f"],
                    b"f" => [b"f", b"foo", b"foobar"],
                    b"b" => [b"baz", b"foobar"],
                    b"a" => [b"baz", b"foobar"]
                };
            }
        }
    }

    describe "the real world" {
        context "5286 files from rust repo" {
            before {
                let files = include_str!("fixtures/rust_repo_files.txt")
                    .lines()
                    .map(|s| s.as_bytes())
                    .collect::<Vec<_>>();

                assert_eq!(files.len(), 5286);
            }

            it "works for `abc`" {
                go! {
                    *files,
                    b"abc" => [
                        b"src/test/compile-fail/attr-bad-crate-attr.rc",
                        b"src/test/run-pass/associated-types-binding-in-where-clause.rs",
                        b"src/test/auxiliary/regions-bounded-method-type-parameters-cross-crate-lib.rs"
                    ]
                };
            }

            it "works for `htn`" {
                go! {
                    *files,
                    b"htn" => [
                        b"src/test/run-pass/hrtb-trait-object-paren-notation.rs",
                        b"src/librustdoc/html/static/normalize.css",
                        b"src/test/compile-fail/match-tag-nullary.rs"
                    ]
                };
            }
        }
    }
}
