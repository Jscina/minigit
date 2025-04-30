use crate::core::utils;
use anyhow::Result;
use std::path::Path;

pub fn run(paths: &[String]) -> Result<()> {
    let ignore_patterns = utils::load_ignore_patterns();
    let mut files_added = 0;

    for path_str in paths {
        let path = Path::new(path_str);

        if path.is_dir() {
            for entry in utils::walk_dir(path, &ignore_patterns)? {
                files_added += utils::stage_file(&entry)?;
            }
        } else if path.is_file() && !utils::should_ignore(path, &ignore_patterns) {
            files_added += utils::stage_file(path)?;
        } else {
            eprintln!("Skipping unrecognized or ignored path: {}", path.display());
        }
    }

    println!("Staged {} files", files_added);
    Ok(())
}
