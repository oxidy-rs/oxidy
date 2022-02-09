use num_cpus;

/*
 * Get Number of CPUs
 */
pub(crate) fn cpus() -> usize {
    let mut n: usize = num_cpus::get();
    if n < num_cpus::get_physical() {
        n = num_cpus::get_physical();
    }
    n
}
