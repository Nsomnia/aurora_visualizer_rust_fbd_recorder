//! General helper functions
//! Version: 0.1.0

use std::path::Path;

pub fn is_valid_preset_file(path: &Path) -> bool {
    // Check if file is a valid ProjectM preset
    // Usually .milk or .prjm files
    
    if let Some(extension) = path.extension() {
        extension == "milk" || extension == "prjm"
    } else {
        false
    }
}

pub fn format_duration(seconds: u32) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}
