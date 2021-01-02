#[allow(dead_code)]
use crate::sys::{FILE_SEP, PATH_SEP};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use zip::ZipArchive;
use log::{error, trace};

static CLASS_PATH_MANAGER: Lazy<RwLock<ClassPathManager>> =
    Lazy::new(|| RwLock::new(ClassPathManager::new()));

pub fn find_class(name: &str) -> Result<ClassPathResult, Error> {
    let cpm = CLASS_PATH_MANAGER.read().unwrap();
    cpm.search_class(name)
}

pub fn add_path(path: &str) {
    let mut cpm = CLASS_PATH_MANAGER.write().unwrap();
    let _ = cpm.add_class_path(path);
}

pub fn add_paths(path: &str) {
    let mut cpm = CLASS_PATH_MANAGER.write().unwrap();
    cpm.add_class_paths(path);
}

#[derive(Debug, Clone)]
pub struct ClassPathResult {
    pub path: String,
    pub data: Vec<u8>,
}

type ZipRef = Arc<Mutex<Box<ZipArchive<File>>>>;

enum ClassSource {
    DIR,
    JAR(ZipRef),
}

struct ClassPathEntry {
    source: ClassSource,
    path: String,
}

struct ClassPathManager {
    class_path: Vec<ClassPathEntry>,
}

impl ClassPathManager {
    fn new() -> Self {
        Self { class_path: vec![] }
    }

    pub fn add_class_path(&mut self, path: &str) -> Result<(), Error> {
        let p = Path::new(path);
        if p.is_dir() {
            self.class_path.push(ClassPathEntry {
                source: ClassSource::DIR,
                path: path.to_string(),
            });
        } else {
            let f = File::open(p)?;
            let z = ZipArchive::new(f)?;
            let handle = Arc::new(Mutex::new(Box::new(z)));
            self.class_path.push(ClassPathEntry {
                source: ClassSource::JAR(handle),
                path: path.to_string(),
            });
        }
        Ok(())
    }

    pub fn add_class_paths(&mut self, path: &str) {
        path.split(PATH_SEP).for_each(|p| {
            if let Err(e) = self.add_class_path(p) {
                error!("add class path error, path = {}, e = {:?}", p, e);
            }
        });
    }

    pub fn search_class(&self, name: &str) -> Result<ClassPathResult, Error> {
        let name = name.replace("/", FILE_SEP);
        let name = name.replace(".", FILE_SEP);
        trace!("search class: {}", name);

        for it in self.class_path.iter() {
            match &it.source {
                ClassSource::DIR => {
                    let mut path = String::from(&it.path);
                    path.push_str(FILE_SEP);
                    path.push_str(&name);
                    path.push_str(".class");
                    if let Ok(data) = std::fs::read(&path) {
                        return Ok(ClassPathResult { path, data });
                    }
                }
                ClassSource::JAR(handle) => {
                    let mut p = String::from(&name);
                    p.push_str(".class");

                    let mut handle = handle.lock().unwrap();
                    let zip_file = handle.by_name(&p);
                    if let Ok(mut zip_file) = zip_file {
                        let mut v = Vec::with_capacity(zip_file.size() as usize);
                        let result = zip_file.read_to_end(&mut v);
                        debug_assert!(result.is_ok());
                        return Ok(ClassPathResult {
                            path: it.path.clone(),
                            data: v,
                        });
                    }
                }
            }
        }

        Err(Error::new(
            ErrorKind::NotFound,
            format!("failed to search class: {}", name),
        ))
    }

    pub fn size(&self) -> usize {
        self.class_path.len()
    }
}
