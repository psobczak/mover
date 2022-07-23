use std::{
    ffi::OsStr,
    fs::{self, File},
    path::Path,
    sync::mpsc::channel,
    time::Duration,
};

use directories::{ProjectDirs, UserDirs};
use duration_string::DurationString;
use fs_extra::{file::move_file, file::CopyOptions};
use log::{info, LevelFilter};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::time::{SystemTime, UNIX_EPOCH};

const EXTENSIONS: [&str; 9] = [
    "png", "jpg", "jpeg", "svg", "tif", "tiff", "bmp", "gif", "eps",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(project_dirs) = ProjectDirs::from("com", "Sobczak", "mover") {
        let config_dir = project_dirs.config_dir();

        fs::create_dir_all(&config_dir)?;

        let log_file_path = config_dir.join("mover.log");

        let log_file = File::options()
            .append(true)
            .create(true)
            .open(log_file_path)
            .unwrap();

        CombinedLogger::init(vec![
            WriteLogger::new(log::LevelFilter::Info, Config::default(), log_file),
            TermLogger::new(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
        ])?;
    }

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1))?;

    if let Some(user_dirs) = UserDirs::new() {
        let downloads_directory = user_dirs.download_dir().unwrap();
        let pictures_directory = user_dirs.picture_dir().unwrap();
        watcher.watch(downloads_directory, RecursiveMode::Recursive)?;

        loop {
            // Listen for create file events in 'downloads' directory
            if let Ok(DebouncedEvent::Create(file_path)) = rx.recv() {
                if let Some(extension) = Path::new(&file_path).extension().and_then(OsStr::to_str) {
                    match EXTENSIONS.contains(&extension) {
                        true => {
                            if let Some(file_name) = file_path.file_stem() {
                                let mut final_path = pictures_directory.join(file_name);
                                let extension =
                                    file_path.extension().and_then(OsStr::to_str).unwrap();

                                // Check if file with given name already exists in 'pictures' directory
                                // Add unix timestamp to the end of file name if it does
                                match pictures_directory.join(file_name).exists() {
                                    true => {
                                        let start = SystemTime::now();
                                        let duration =
                                            DurationString::from(start.duration_since(UNIX_EPOCH)?)
                                                .to_string();

                                        let mut new_file_name = String::new();
                                        new_file_name.push_str(OsStr::to_str(file_name).unwrap());
                                        new_file_name.push('_');
                                        new_file_name.push_str(&duration);
                                        new_file_name.push('.');
                                        new_file_name.push_str(extension);

                                        final_path = pictures_directory
                                            .join(pictures_directory.join(&new_file_name));

                                        move_file(
                                            &file_path,
                                            &final_path,
                                            &CopyOptions::default(),
                                        )?;
                                        info!(
                                            "Moved {} to: {}",
                                            &new_file_name,
                                            final_path.display()
                                        );
                                    }
                                    false => {
                                        move_file(
                                            &file_path,
                                            &final_path,
                                            &CopyOptions::default(),
                                        )?;
                                        info!(
                                            "Moved {} to: {}",
                                            &file_name.to_str().unwrap(),
                                            &pictures_directory.join(&file_path).display()
                                        );
                                    }
                                };
                            }
                        }
                        false => info!("File {} is not an image", &file_path.display()),
                    }
                }
            }
        }
    }
    Ok(())
}
