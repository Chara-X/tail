use notify::Watcher;
use std::{
    fs,
    io::{self, Seek},
    ops,
    sync::mpsc,
};
pub struct File {
    file: fs::File,
    path: String,
}
impl File {
    pub fn open(path: &str) -> io::Result<File> {
        Ok(File {
            file: fs::File::open(path)?,
            path: path.to_string(),
        })
    }
    pub fn fellow<W: io::Write>(&self, mut writer: W) -> notify::Result<()> {
        let mut reader = io::BufReader::new(&self.file);
        reader.seek(io::SeekFrom::End(0))?;
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        watcher.watch(self.path.as_ref(), notify::RecursiveMode::NonRecursive)?;
        for event in rx {
            if event?.kind.is_modify() {
                io::copy(&mut reader, &mut writer)?;
                writer.flush()?;
            }
        }
        Ok(())
    }
}
impl ops::Deref for File {
    type Target = fs::File;
    fn deref(&self) -> &Self::Target {
        &self.file
    }
}
