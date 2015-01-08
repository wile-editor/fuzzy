#![feature(plugin)]

#[plugin]
extern crate speculate;

extern crate fuzzy;

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
            assert_eq!(2, ::fuzzy::score(b"aaa", b"aa"));
            assert_eq!(1, ::fuzzy::score(b"abc", b"a"));
            assert_eq!(2, ::fuzzy::score(b"xyza", b"xa"));
        }
    }

    describe "sorted" {
        it "works!" {
            assert_eq!([b"foo", b"bar"],
                       ::fuzzy::sorted(&[b"bar", b"foo"], b"f"));
        }
    }
}
