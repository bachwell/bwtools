#![allow(dead_code)]
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ApplicationDirs {
    project_dirs: Option<directories::ProjectDirs>,
    home: PathBuf,
}

impl ApplicationDirs {
    pub fn new(qualifier: &str, organization: &str, application: &str) -> Self {
        let project_dirs = directories::ProjectDirs::from(qualifier, organization, application);
        let home = directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_path_buf();

        Self {
            project_dirs: project_dirs,
            home: home,
        }
    }

    pub fn home(&self) -> &Path {
        self.home.as_ref()
    }

    pub fn data(&self) -> &Path {
        let res = self.project_dirs.as_ref().unwrap().data_local_dir();
        res
    }

    pub fn config(&self) -> &Path {
        self.project_dirs
            .as_ref()
            .unwrap()
            .config_local_dir()
            .as_ref()
    }

    pub fn print_dirs(&self) {
        if let Some(ref proj_dir) = self.project_dirs {
            log::debug!("Project data: {:?}", &proj_dir.data_dir());
            log::debug!("Project data local: {:?}", &proj_dir.data_local_dir());
            log::debug!("Project cache: {:?}", &proj_dir.cache_dir());
            log::debug!("Project config: {:?}", &proj_dir.config_dir());
            log::debug!("Project config local: {:?}", &proj_dir.config_local_dir());
            log::debug!("Project preference: {:?}", &proj_dir.preference_dir());
            log::debug!("Project project path: {:?}", &proj_dir.project_path());
            log::debug!("Project state: {:?}", &proj_dir.state_dir());
        }

        if let Some(user_dirs) = directories::UserDirs::new() {
            log::debug!("User home: {:?}", &user_dirs.home_dir());
            log::debug!("User desktop: {:?}", &user_dirs.desktop_dir());
            log::debug!("User download: {:?}", &user_dirs.download_dir());
            log::debug!("User public: {:?}", &user_dirs.public_dir());
            log::debug!("User font: {:?}", &user_dirs.font_dir());
            log::debug!("User document: {:?}", &user_dirs.document_dir());
            log::debug!("User template: {:?}", &user_dirs.template_dir());
            log::debug!("User audio: {:?}", &user_dirs.audio_dir());
            log::debug!("User picture: {:?}", &user_dirs.picture_dir());
            log::debug!("User video: {:?}", &user_dirs.video_dir());
        }
    }
}
