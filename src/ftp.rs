use std::io::{BufReader, BufRead};
use std::io::Write;
use std::net::TcpStream;

pub struct Ftp {
    stream: TcpStream,
}

impl Ftp {
    /// Generate an FTP struct
    pub fn new(stream: TcpStream) -> Self {
        Ftp {
            stream: stream,
        }
    }

    /// Begin the FTP session
    pub fn begin(&mut self) {
        self.stream.write(b"220\n").unwrap();
        self.anonymous_login();
        loop {
            let message = self.read();
            self.parse_command(&message);
        }
    }

    /// Parse the given message into an action on the FTP server
    fn parse_command(&mut self, message: &String) {
        // Parse the given message into an action
    }

    /// When we need anonymous login perform the steps to authenticate the user
    fn anonymous_login(&mut self) {
        let resp = self.read();
        self.write(331, "Please specify the password");
        let resp = self.read();
        self.write(230, "Login successful");
        self.stream.write(b"230 Login successful\n").unwrap();
        self.report_system_type();
    }

    /// Report the system type the server is running on
    fn report_system_type(&mut self) {
        self.write(215, "UNIX");
    }

    /// Accept message from the client
    fn read(&mut self) -> String {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        buffer
    }

    /// Write a message to the client in command mode
    fn write(&mut self, code: i32, command: &str) {
        let message = if command.chars().last().unwrap() != '\n' {
            format!("{} {}\n", code, command)
        } else {
            format!("{} {}", code, command)
        };
        self.stream
            .write(message.as_bytes())
            .unwrap();
    }
}
