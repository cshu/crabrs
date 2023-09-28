//this function is for allocating filename for huge amount of files. Each file has a id (unsigned integer). And no folder will contain too many files. The tree goes deeper and deeper when total number of files increase.
pub fn mk_treefile_name(file_id: u32, dirname: &mut std::path::PathBuf) {
    let mut idstr = file_id.to_string();
    idstr.push('a');
    for idx in 0.. {
        if idx == idstr.len() - 2 {
            dirname.push(&idstr[idx..]);
            break;
        } else {
            dirname.push(&idstr[idx..idx + 1]);
        }
    }
}
