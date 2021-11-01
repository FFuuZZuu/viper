use std::fmt;

pub enum Error {
    IllegalSyntaxError {
        found: String,
        filepath: String,
        coord: (u32, u32),
    },
    FoundExpectedError {
        found: String,
        expected: String,
        filepath: String,
        coord: (u32, u32),
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IllegalSyntaxError {
                found,
                filepath,
                coord,
            } => Error::handle_illegal_syntax_error(f, found.clone(), filepath.clone(), *coord),
            Error::FoundExpectedError {
                found,
                expected,
                filepath,
                coord,
            } => Error::handle_found_expected_error(
                f,
                found.clone(),
                expected.clone(),
                filepath.clone(),
                *coord,
            ),
        }
    }
}

impl Error {
    fn handle_illegal_syntax_error(
        f: &mut fmt::Formatter<'_>,
        found: String,
        filepath: String,
        coord: (u32, u32),
    ) -> fmt::Result {
        write!(f, "ERROR: Illegal Syntax Error\n")?;
        write!(f, "  -->  {}:{}:{}\n", filepath, coord.0, coord.1)?;

        write!(f, "Found: {}", found)
        // TODO: Error messages with pointers
        /*
        write!(f, "{:4} |       ", row)?;

        let mut current = '\0';
        while current != '\n' && current != '\r' && file.len() > 0 {
            current = file.remove(0);
            write!(f, "{}", current)?;
        }

        let mut whitespace = "".to_string();
        for _ in 0..col - 1 {
            whitespace.push(' ');
        }
        write!(f, "     |       {}^", whitespace)
        */
    }

    fn handle_found_expected_error(
        f: &mut fmt::Formatter<'_>,
        found: String,
        expected: String,
        filepath: String,
        coord: (u32, u32),
    ) -> fmt::Result {
        write!(f, "ERROR: Invalid Syntax Error\n")?;
        write!(f, "  -->  {}:{}:{}\n", filepath, coord.0, coord.1)?;

        write!(f, "Found {}, Expected {}", found, expected)
    }
}
