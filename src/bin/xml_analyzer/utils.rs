use std::{
    io,
    path::{Path, PathBuf},
};

#[expect(dead_code)]
pub(super) fn get_new_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    // パスが存在しない場合はそのまま返す
    if !path.exists() {
        return path.to_path_buf();
    }

    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|s| s.to_str());
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    let mut counter = 1;
    loop {
        // 新しいファイル名を生成
        let new_filename = match ext {
            Some(e) => format!("{}_{}.{}", stem, counter, e),
            None => format!("{}_{}", stem, counter),
        };

        let new_path = parent.join(new_filename);

        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

pub(super) fn ls_xml<P: AsRef<Path>>(path: P) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(std::fs::read_dir(path)?.flatten().filter_map(|entry| {
        if entry
            .path()
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
        {
            Some(entry.path())
        } else {
            None
        }
    }))
}
