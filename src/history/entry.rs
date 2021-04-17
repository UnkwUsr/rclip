use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
pub struct HistoryEntry {
    data: Vec<u8>,
    target_name: String,
    is_data_in_file: bool,
}

/// Entry format:
/// ```
/// TARGET_NAME DATA_LENGTH [! (is is data in file)]
/// ENTRY_DATA/FILE_NAME
/// ```
/// Example of entry:
/// ```
/// UTF8_STRING 17
/// some_example_text
/// ```
/// Example of entry which data stored in file 1618508972208 (relative to target folder):
/// ```
/// image/png 13 !
/// 1618508972208
/// ```
impl HistoryEntry {
    pub fn next_from_bufreader(bufreader: &mut BufReader<File>) -> Option<Self> {
        let mut meta_info_str = String::new();
        bufreader.read_line(&mut meta_info_str).unwrap();
        if meta_info_str.is_empty() {
            None
        } else {
            let meta_info: Vec<&str> = meta_info_str.split_ascii_whitespace().collect();

            if meta_info.len() < 2 || meta_info.len() > 3 {
                panic!("Bad formatted meta info: {}", meta_info_str)
            } else {
                let target_name = meta_info[0].to_owned();
                let data_len = meta_info[1].parse::<usize>().unwrap();
                let is_data_in_file = if meta_info.len() == 3 && meta_info[2] == "!" {
                    true
                } else {
                    false
                };
                let mut data = vec![0; data_len + 1];

                bufreader.read_exact(&mut data).unwrap();
                // drop traling new line
                data.pop();

                Some(HistoryEntry {
                    data,
                    target_name,
                    is_data_in_file,
                })
            }
        }
    }
}

impl std::fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let mut v = self.data.clone();

        if f.alternate() {
            for x in &mut v {
                if *x == b'\n' {
                    *x = b' '
                }
            }
            write!(f, "{}", String::from_utf8(v).unwrap())?;
            Ok(())
        } else {
            writeln!(
                f,
                "{}{}",
                if self.is_data_in_file { "! " } else { "" },
                self.target_name
            )?;
            write!(f, "{}", String::from_utf8(v).unwrap())?;

            Ok(())
        }
    }
}
