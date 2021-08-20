use crate::time_tracker_store::{TimeTrackerStore, Event};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Write, Read, Seek, SeekFrom};

pub struct FileDatabase {
    file: File
}

impl TimeTrackerStore for FileDatabase {
    fn prepare(profile_name: &str) -> FileDatabase {
        FileDatabase {
            file: get_handler(profile_name)
        }
    }

    fn add_topic(&mut self, topic: &str, time: u128) {
        write_new_topic(&mut self.file, topic, time)
    }

    fn retrieve_events(&mut self) -> Vec<crate::time_tracker_store::Event> {
        let mut file_contents= String::new();
        self.file.seek(SeekFrom::Start(0)).expect("Moving file cursor failed");
        self.file.read_to_string(&mut file_contents).expect("Reading file contents failed");
        self.file.seek(SeekFrom::End(0)).expect("Moving file cursor failed");
        return get_events_from_file_contents(&file_contents);
    }
}

fn get_handler(profile_name: &str) -> File {
    let file_name = format!("time_data_{}.txt", profile_name);
    let path = Path::new(&file_name);
    let display = path.display();

    return match OpenOptions::new().create(true).append(true).read(true).open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create {}: {}", display, why),
    };
}

fn write_new_topic(file_handler: &mut File, topic: &str, time: u128 ) {
    let line: String = format!("{}; {}\n", time.to_string(), &topic.trim());
    file_handler.write(line.as_ref()).expect("Writing to file failed!");
}

fn get_events_from_file_contents(file_contents: &str) -> Vec<Event> {
    let mut events = Vec::new();
    for line in file_contents.split("\n") {
        if line != "" {
            events.push(get_event_from_line(line) );
        }
    }
    return events;
}

fn get_event_from_line(line: &str) -> Event {
    let parts : Vec<&str> = line.split(";").collect();
    let topic = parts[1].trim().to_string();
    let start_time = parts[0].to_string().parse::<u128>().unwrap();

    return Event {
        start_time,
        topic
    }
}

#[cfg(test)]
mod tests {
    use crate::file_database::{get_handler, write_new_topic, get_events_from_file_contents, FileDatabase};
    use std::path::Path;
    use std::io::Read;
    use std::fs::File;
    use crate::time_tracker_store::{Event, TimeTrackerStore};

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
        assert_eq!(file_content, "12; TestTopic\n");

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
        assert_eq!(file_content, "12; TestTopic\n22; TestTopic2\n");

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
        assert_eq!(file_content, "16; TestTopic\n");

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
        assert_eq!(file_content, "20; TestTopicWithoutNewline\n");

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
        assert_eq!(file_content, "19; TestTopicWithNewline\n");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_get_events_from_file_string_correctly() {
        let input = "\
            111; TestString\n\
            1234; MoreTest\n\
        ";
        let expected = vec![
            Event {
                topic: "TestString".to_string(),
                start_time: 111
            },
            Event {
                topic: "MoreTest".to_string(),
                start_time: 1234
            }
        ];

        let actual = get_events_from_file_contents(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_get_events_from_file_correctly() {
        let mut database_handler = FileDatabase::prepare("get_events_tests");
        let path = Path::new("time_data_get_events_tests.txt");

        database_handler.add_topic( "RealWrite", 29);
        database_handler.add_topic( "Another", 31);


        let expected = vec![
            Event {
                topic: "RealWrite".to_string(),
                start_time: 29
            },
            Event {
                topic: "Another".to_string(),
                start_time: 31
            }
        ];
        let actual = database_handler.retrieve_events();

        assert_eq!(actual, expected);

        std::fs::remove_file(path).unwrap();
    }
}