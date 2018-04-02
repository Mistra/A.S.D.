mod quicksort;
mod mergesort;
mod insertsort;
mod selectionsort;
mod heapsort;
mod tournament;

pub use self::quicksort::quicksort;
pub use self::mergesort::mergesort;
pub use self::insertsort::{
    insertsort,
    r_insertsort
};
pub use self::selectionsort::selectionsort;
pub use self::heapsort::heapsort;
pub use self::tournament::tournament;

#[cfg(test)]
mod test_common;
