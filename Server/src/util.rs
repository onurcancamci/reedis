pub fn next_path_key(path: (&str, usize)) -> (&str, Option<usize>) {
    let p_ind = path.0[path.1..].find('/');
    match p_ind {
        Some(p_ind) => ((&path.0[path.1..(path.1 + p_ind)], Some(path.1 + p_ind + 1))),
        None => (&path.0[path.1..], None),
    }
}
