use num_cpus::{get, get_physical};

/*
 * Get Number of CPUs
 */
pub(crate) fn cpus() -> usize {
    let mut n: usize = get();
    if n < get_physical() {
        n = get_physical();
    }
    n
}
