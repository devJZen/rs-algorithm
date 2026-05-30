pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod intro;
pub mod merge;
pub mod quick;
pub mod selection;
pub mod shell;
pub mod tim;

use crate::cli::Algorithm;

pub type Step = (Vec<i32>, usize, usize);

pub fn generate_steps(alg: &Algorithm, data: &[i32]) -> Vec<Step> {
    match alg {
        Algorithm::Bubble => bubble::steps(data),
        Algorithm::Selection => selection::steps(data),
        Algorithm::Insertion => insertion::steps(data),
        _ => vec![(data.to_vec(), 0, 0)],
    }
}

pub fn explanation(alg: &Algorithm) -> &'static str {
    match alg {
        Algorithm::Bubble => bubble::EXPLANATION,
        Algorithm::Selection => selection::EXPLANATION,
        Algorithm::Insertion => insertion::EXPLANATION,
        Algorithm::Merge => merge::EXPLANATION,
        Algorithm::Quick => quick::EXPLANATION,
        Algorithm::Heap => heap::EXPLANATION,
        Algorithm::Shell => shell::EXPLANATION,
        Algorithm::Tim => tim::EXPLANATION,
    }
}
