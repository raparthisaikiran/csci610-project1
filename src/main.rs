use chrono::{DateTime, Local, LocalResult, TimeZone};
use filetime::FileTime;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::{
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
    vec,
};
use walkdir::WalkDir;
/**
 * @author Raparthisaikiran
 */
fn main() {
    let path = "data";
    // get list of directories within `path`
    let dir_list = read_dir(path);

    // get list of files within the first directory
    let dir1_files = read_files_from_dir(dir_list[0].clone().as_str());
    // get list of files within the second directory
    let dir2_files = read_files_from_dir(dir_list[1].clone().as_str());

    // get list of paired files (i.e. files with same names in both directories)
    let paired_files = get_paired_files(dir1_files.clone(), dir2_files.clone());
    // get list of unpaired files in the first directory
    let unpaired_files1 = get_unpaired_files(dir1_files.clone(), dir2_files.clone());
    // get list of unpaired files in the second directory
    let unpaired_files2 = get_unpaired_files(dir2_files.clone(), dir1_files.clone());

    // refactor the names of the paired files
    refactor_paired_file(paired_files.clone(), dir_list.clone());
    // refactor the names of the unpaired files in the first directory
    refactor_unpaired_files(
        unpaired_files1,
        dir_list[0].clone().as_str(),
        paired_files.len(),
    );
    // refactor the names of the unpaired files in the second directory
    refactor_unpaired_files(
        unpaired_files2,
        dir_list[1].clone().as_str(),
        paired_files.len(),
    );
}

/**
 * Read's the child directories from a given path
 *
 * @params grand parent path
 * @returns Vector of directories
 */
fn read_dir(path: &str) -> Vec<String> {
    let mut dir = vec![];
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.metadata().unwrap().is_dir() && entry.path().display().to_string() != path {
            dir.push(entry.path().display().to_string());
        }
    }
    dir
}

/**
 * Read's the all files from a given path
 *
 * @params path: path of directory to read files
 * @returns Vector of filenames
 */

fn read_files_from_dir(path: &str) -> Vec<String> {
    let mut files = vec![];
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.metadata().unwrap().is_file() {
            files.push(
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    files
}

/**
 * Get list of paired files (i.e. files with same names in both directories)
 *
 * @params files1: Vector of filenames from directory 1
 * @params files2: Vector of filenames from directory 2
 * @returns Vector of paired filenames
 */
fn get_paired_files(files1: Vec<String>, files2: Vec<String>) -> Vec<String> {
    let mut paired_files: Vec<String> = Vec::new();
    for value in files1 {
        if files2.contains(&value) {
            paired_files.push(value);
        }
    }
    paired_files
}

/**
 * Get list of unpaired files (i.e. directory 1 files with different names in directory 2)
 *
 * @params files1: Vector of filenames from directory 1
 * @params files2: Vector of filenames from directory 2
 * @returns Vector of unpaired filenames
 */
fn get_unpaired_files(files1: Vec<String>, files2: Vec<String>) -> Vec<String> {
    let mut unpaired_files: Vec<String> = Vec::new();
    for value in files1 {
        if !files2.contains(&value) {
            unpaired_files.push(value);
        }
    }
    unpaired_files
}

/**
 * @description This function takes a vector of paired file names and a vector of directory paths and renames the files with a randomized name. 
 * The files are renamed using temporary names first and then the randomized name in the second loop
 @params
 */
fn refactor_paired_file(paired_files: Vec<String>, dir_list: Vec<String>) {
    let names = shuffle_numbers(1, paired_files.len().try_into().unwrap());
    for index in 0..paired_files.len() {
        let file: Vec<_> = paired_files[index].split('.').collect();
        let temp = "temp_name".to_owned() + &index.to_string();
        rename_file(dir_list[0].as_str(), file[0], file[1], &temp, false);
        rename_file(dir_list[1].as_str(), file[0], file[1], &temp, false);
    }
    for index in 0..paired_files.len() {
        let file: Vec<_> = paired_files[index].split('.').collect();
        let temp = "temp_name".to_owned() + &index.to_string();
        print_log(
            &dir_list[0].as_str(),
            &temp,
            &file[1],
            paired_files[index].clone(),
        );
        rename_file(
            dir_list[0].as_str(),
            &temp,
            file[1],
            names[index].to_string().as_str(),
            true,
        );
        print_log(
            &dir_list[1].as_str(),
            &temp,
            &file[1],
            paired_files[index].clone(),
        );
        rename_file(
            dir_list[1].as_str(),
            &temp,
            file[1],
            names[index].to_string().as_str(),
            true,
        );
    }
}

/**
 * @description This function takes a vector of unpaired file names, the directory path, and the number of files in the directory
 * It renames the files with a randomized name. The files are renamed using temporary names first and then the randomized name in the second loop
  @params
 */
fn refactor_unpaired_files(unpaired_files: Vec<String>, dir: &str, len: usize) {
    let names = shuffle_numbers(len as u32 + 1, len as u32 + unpaired_files.len() as u32);
    for index in 0..unpaired_files.len() {
        let file: Vec<_> = unpaired_files[index].split('.').collect();
        let temp = "temp_name".to_owned() + &index.to_string();
        rename_file(dir, file[0], file[1], &temp, false);
    }
    for index in 0..unpaired_files.len() {
        let file: Vec<_> = unpaired_files[index].split('.').collect();
        let temp = "temp_name".to_owned() + &index.to_string();
        print_log(&dir, &temp, &file[1], unpaired_files[index].clone());
        rename_file(dir, &temp, file[1], names[index].to_string().as_str(), true);
    }
}

/**
 * @description This function renames a file with a new name. It takes the directory path, the file name, 
 * the file extension, the new name, and a boolean to decide whether to log the change
 * @params
 */
fn rename_file(dir_name: &str, file_name: &str, extension: &str, new_name: &str, log: bool) {
    let old_path = format!("{}/{}.{}", dir_name, file_name, extension);
    let new_path = format!("{}/{}.{}", dir_name, new_name, extension);
    let new_time = random_time(&PathBuf::from(old_path.clone()));
    match fs::rename(&old_path, &new_path) {
        Ok(_) => {
            let new_time = new_time.unwrap();
            filetime::set_file_times(&new_path, new_time, new_time).unwrap();
            if log {
                println!(
                    "New file name: {}, New file time: {:?}",
                    new_path,
                    convert_time_format(new_time)
                );
                println!();
            }
        }
        Err(e) => println!("Error renaming {}: {}", file_name, e),
    };
}

/**
 * @descriptor This function shuffles a range of numbers and returns them as a vector
 * It takes the start and end numbers of the range as arguments
 */
fn shuffle_numbers(start: u32, end: u32) -> Vec<u32> {
    let mut numbers: Vec<u32> = (start..=end).collect();
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);
    numbers
}

/**
 * @descriptor This function generates a random time between 1 hour and 10 days ago for a given file
 * It takes the file path as an argument and returns the new time as an Option<FileTime>
 * @params
 */
fn random_time(file: &PathBuf) -> Option<FileTime> {
    const SEC_IN_10_DAYS: i64 = 10 * 24 * 60 * 60;
    const SEC_IN_HOUR: i64 = 60 * 60;
    if let Ok(_metadata) = file.metadata() {
        let now = SystemTime::now();
        let rand_secs = SEC_IN_HOUR + thread_rng().gen_range(0..=(SEC_IN_10_DAYS - SEC_IN_HOUR));
        let new_time = FileTime::from_system_time(now - Duration::from_secs(rand_secs as u64));
        return Some(new_time);
    }
    None
}

/**
 * @descriptor This function gets the original modification time of a file
 * It takes the file path as an argument and returns the original time as an Option<FileTime>
 * @params
 */
fn get_original_time(file: &PathBuf) -> Option<FileTime> {
    if let Ok(metadata) = file.metadata() {
        let original_time = FileTime::from_last_modification_time(&metadata);
        return Some(original_time);
    }
    None
}

/**
 * @descriptor This function converts a FileTime to a DateTime in the Local timezone
 * It takes the FileTime as an argument and returns a LocalResult<DateTime<Local>>
 * @params
 */
fn convert_time_format(time: FileTime) -> LocalResult<DateTime<Local>> {
    let format_time = Local.timestamp_opt(time.seconds(), time.nanoseconds() as u32);
    format_time
}

/**
 * @descriptor This function prints a log message with the old file name, old file time, and new file name and time
 * It takes the directory path, temporary file name, file extension, and original file name as arguments
 * @params
 */
fn print_log(dir: &str, filename: &str, extension: &str, original_name: String) {
    let path = format!("{}/{}.{}", dir, filename, extension);
    let time = get_original_time(&PathBuf::from(path.clone()));
    let formatted_time = convert_time_format(time.unwrap());
    println!(
        "Old file name: {}/{}, Old file time: {:?}",
        &dir, &original_name, formatted_time
    );
}
