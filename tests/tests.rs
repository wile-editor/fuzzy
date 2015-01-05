#![feature(phase)]

#[phase(plugin)]
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
    }
}
