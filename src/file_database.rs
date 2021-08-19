use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Write;

pub fn get_handler(profile_name: &str) -> File {
    let file_name = format!("time_data_{}.txt", profile_name);
    let path = Path::new(&file_name);
    let display = path.display();

    return match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create {}: {}", display, why),
    };

}

pub fn write_new_topic(file_handler: &mut File, topic: &str, time: u128 ) {
    let line: String = format!("{}: {}\n", time.to_string(), &topic.trim());
    file_handler.write(line.as_ref()).expect("Writing to file failed!");
}

#[cfg(test)]
mod tests {
    use crate::file_database::{get_handler, write_new_topic};
    use std::path::Path;
    use std::io::Read;
    use std::fs::File;

    #[test]
    fn should_create_file() {
        let _file = get_handler("file_creation");

        let path = Path::new("time_data_file_creation.txt");
        std::fs::remove_file(path).expect("Removing file failed!");
    }

    #[test]
    fn should_write_to_file() {
        let mut file_handler = get_handler("file_single_write");
        let path = Path::new("time_data_file_single_write.txt");
        let mut file_content = String::new();

        write_new_topic(&mut file_handler, "TestTopic", 12);

        let mut file_reader = File::open(path).unwrap();
        file_reader.read_to_string(&mut file_content).expect("File reading failed");
        assert_eq!(file_content, "12: TestTopic\n");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_write_multiple_lines_to_file() {
        let mut file_handler = get_handler("file_double_write");
        let path = Path::new("time_data_file_double_write.txt");
        let mut file_content = String::new();

        write_new_topic(&mut file_handler, "TestTopic", 12);
        write_new_topic(&mut file_handler, "TestTopic2", 22);

        let mut file_reader = File::open(path).unwrap();
        file_reader.read_to_string(&mut file_content).expect("File reading failed");
        assert_eq!(file_content, "12: TestTopic\n22: TestTopic2\n");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_open_in_append_mode() {
        let mut file_handler = get_handler("file_append_mode");
        let path = Path::new("time_data_file_append_mode.txt");
        let mut file_content = String::new();

        write_new_topic(&mut file_handler, "TestTopic", 16);

        get_handler("file_append_mode");

        let mut file_reader = File::open(path).unwrap();
        file_reader.read_to_string(&mut file_content).expect("File reading failed");
        assert_eq!(file_content, "16: TestTopic\n");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_have_line_break_after_writing() {
        let mut file_handler = get_handler("has_newlines");
        let path = Path::new("time_data_has_newlines.txt");
        let mut file_content = String::new();

        write_new_topic(&mut file_handler, "TestTopicWithoutNewline", 20);

        let mut file_reader = File::open(path).unwrap();
        file_reader.read_to_string(&mut file_content).expect("File reading failed");
        assert_eq!(file_content, "20: TestTopicWithoutNewline\n");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_have_exactly_one_line_break_after_writing() {
        let mut file_handler = get_handler("clean_newlines");
        let path = Path::new("time_data_clean_newlines.txt");
        let mut file_content = String::new();

        write_new_topic(&mut file_handler, "TestTopicWithNewline\n", 19);

        let mut file_reader = File::open(path).unwrap();
        file_reader.read_to_string(&mut file_content).expect("File reading failed");
        assert_eq!(file_content, "19: TestTopicWithNewline\n");

        std::fs::remove_file(path).unwrap();
    }
}