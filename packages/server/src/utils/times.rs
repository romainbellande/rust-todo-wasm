pub fn times<T, F>(nb: u32, f: F) -> Vec<T>
where
    F: FnMut(u32) -> T,
{
    (0..nb).into_iter().map(f).collect()
}
