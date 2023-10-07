use poll_promise::Promise;

pub(crate) struct SelectedFile {
    file_name: String,
    content: Vec<u8>,
}

impl SelectedFile {
    pub(crate) fn file_name(&self) -> &str {
        self.file_name.as_ref()
    }

    pub(crate) fn content(&self) -> &[u8] {
        self.content.as_ref()
    }
}

#[derive(Default)]
pub(crate) struct AsyncFileSelector {
    file: Option<Promise<Option<SelectedFile>>>,
}

pub(crate) enum FileResult {
    NotSelected,
    Opening,
    Ok(SelectedFile),
    FileError,
}

impl AsyncFileSelector {
    pub(crate) fn new() -> Self {
        Self { file: None }
    }

    pub(crate) fn select(&mut self) {
        self.file = read();
    }

    pub(crate) fn get_file(&mut self) -> FileResult {
        if let Some(a) = self.file.take() {
            match a.try_take() {
                Ok(b) => {
                    if let Some(d) = b {
                        FileResult::Ok(d)
                    } else {
                        FileResult::FileError
                    }
                }
                Err(c) => {
                    self.file = Some(c);
                    FileResult::Opening
                }
            }
        } else {
            FileResult::NotSelected
        }
    }
}

impl From<FileResult> for Option<SelectedFile> {
    fn from(val: FileResult) -> Self {
        match val {
            FileResult::Ok(f) => Some(f),
            _ => None,
        }
    }
}

pub(crate) fn read() -> Option<Promise<Option<SelectedFile>>> {
    #[cfg(not(target_arch = "wasm32"))]
    return read_native();
    #[cfg(target_arch = "wasm32")]
    return read_web();
}

#[cfg(not(target_arch = "wasm32"))]
fn read_native() -> Option<Promise<Option<SelectedFile>>> {
    let (sender, promise) = Promise::new();

    if let Some(path) = rfd::FileDialog::new().pick_file() {
        let result =
            if let (Some(path), Ok(data)) = (&path.file_name(), std::fs::read(path.clone())) {
                Some(SelectedFile {
                    file_name: path.to_string_lossy().into(),
                    content: data,
                })
            } else {
                None
            };
        sender.send(result);
        Some(promise)
    } else {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn read_web() -> Option<Promise<Option<SelectedFile>>> {
    Some(Promise::spawn_async(async {
        if let Some(file_handle) = rfd::AsyncFileDialog::new().pick_file().await {
            Some(SelectedFile {
                file_name: file_handle.file_name(),
                content: file_handle.read().await,
            })
        } else {
            None
        }
    }))
}
