use notify::event::{ModifyKind, RenameMode};
use notify::EventKind::Modify;
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};
use zip::ZipArchive;

#[cfg(target_os = "linux")]
const FELLOW: &'static [u8] = include_bytes!("fellow.bmp");

fn main() -> Result<(), Box<dyn Error>> {
    let directories = directories::UserDirs::new().expect("Failed to get directories info");
    let download_dir = directories.download_dir();

    if download_dir.is_none() {
        println!("No downloads directory found");
        return Ok(());
    }

    let args = env::args().skip(1).next();

    let watch_dir = download_dir.unwrap();
    let output_dir = if let Some(p) = args.map(PathBuf::from) {p} else {
        println!("Output directory not provided");
        return Ok(())
    };

    if !watch_dir.exists() {
        println!("watch directory {} does not exist", watch_dir.to_string_lossy());
        return Ok(());
    }else {
        println!("Watching {}", watch_dir.to_string_lossy());
    }

    if !output_dir.exists() {
        println!("output directory {} does not exist", output_dir.to_string_lossy());
        return Ok(());
    } else {
        println!("Outputting to {}", output_dir.to_string_lossy());
    }

    let mut watcher = recommended_watcher(move |event: notify::Result<Event>|match event {
        Ok(e) => {
            if let Modify(ModifyKind::Name(RenameMode::Both)) = e.kind {
                let archive_path = &e.paths[1];
                if let Ok(f) = File::open(archive_path).map(|f|BufReader::new(f)) {
                    if let Ok(mut archive) = ZipArchive::new(f) {
                        if archive.file_names().find(|f|f.to_ascii_lowercase() == "info.dat").is_none() {
                            println!("Not beatsaber file");
                            return;
                        }
                        println!("Extracting {}", archive_path.to_string_lossy());
                        let folder_name = archive_path.file_stem();

                        if folder_name.is_none() {
                            println!("Failed to extract folder name from {}", archive_path.to_string_lossy());
                            return;
                        }

                        let output_dir = output_dir.join(folder_name.expect("Expected a .zip file, found no extension"));
                        let _ = archive.extract(&output_dir);
                        println!("Extracted to {}", output_dir.to_string_lossy())
                    }
                };
            }
        }
        Err(err) => println!("Watcher error {err}")
    })?;

    watcher.watch(&watch_dir, RecursiveMode::NonRecursive)?;

    let (tx, rx) = mpsc::channel();
    #[cfg(target_os = "windows")]
    let mut tray = TrayItem::new("Helpful Hyrax", IconSource::Resource("fellow.bmp"))?;


    #[cfg(target_os = "linux")]
    let mut tray = {
        let icon = image::load_from_memory(FELLOW)?.to_rgba8();
        let icon_argb = icon.as_ref().chunks(4).flat_map(|rgba|[rgba[3], rgba[0], rgba[1], rgba[2]]).collect();
        TrayItem::new("Helpful Hyrax", IconSource::Data {width: icon.width() as i32, height: icon.height() as i32, data: icon_argb})?
    };

    tray.add_label("Helpful Hyrax")?;
    let _ = tray.add_menu_item("Quit", move || {
        let _ = tx.send(());
    });

    let _ = rx.recv();
    Ok(())
}
