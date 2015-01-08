#![feature(plugin)]

#[plugin]
extern crate speculate;

extern crate fuzzy;

macro_rules! go {
    ($haystacks:expr, $($needle:expr => $expected:expr),+) => {{
        $(
            let haystacks = &$haystacks;
            let got = ::fuzzy::sorted(haystacks, $needle);
            assert_eq!($expected, &got[..$expected.len()]);
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

        context "bonus" {
            context "is given to" {
                it "first char match" {
                    go! {
                        [b"barfoo", b"foobar"],
                        b"b" => [b"barfoo", b"foobar"],
                        b"f" => [b"foobar", b"barfoo"]
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
    }
}
