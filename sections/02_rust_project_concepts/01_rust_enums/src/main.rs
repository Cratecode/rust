// One of the functions below makes use of
// random numbers using the rand crate.
// If you want to use this in your projects, you need
// to install it with `cargo add rand`.
use rand::prelude::*;

// This is an enum.
// It's a data type that can be in different states (called variants),
// which are listed below (and separated by commas).
// In Rust, enums can also hold data, so if it's in the Success state,
// it also has the data that got returned.
// On the other hand, if an error occurred, it contains the error code.
// For the first two states, we don't have any data yet, so they
// don't contain any.

/// The status of a request.
#[derive(Debug, PartialEq)]
pub enum Status {
    /// The request has just begun.
    Started,
    /// The request is running but hasn't been completed yet.
    InProgress,
    /// The request succeeded.
    /// Contains the response string.
    Success(String),
    /// The request failed.
    /// Contains the error code.
    Error(u32),
}

// This trait implementation allows converting from
// Status to Option<Result<String, u32>>.
// If it's convertible (if it's a Success or an Error), we'll
// return Some(Result<String, u32>), otherwise None.
// Doing this lets us do status.into().

impl From<Status> for Option<Result<String, u32>> {
    // Self here means the type that we're implemented on,
    // or Option<Result<String, u32>>.
    fn from(status: Status) -> Self {
        match status {
            // We can use | as an or to run some code
            // if any of the patterns match.
            // This means if it's equal to started or in progress.
            // None is equivalent to null in other languages.
            Status::Started | Status::InProgress => None,
            Status::Success(msg) => Some(Ok(msg)),
            Status::Error(code) => Some(Err(code)),
        }
    }
}

/// Starts a simulated request.
pub fn start_request() -> Status {
    Status::Started
}

/// Advances a simulated request.
pub fn advance_request(status: &mut Status) {
    match status {
        // If we're started, change to in progress.
        Status::Started => {
            // We use the dereference (*) to set the value
            // behind the reference.
            // If we didn't use it, then it would try to set
            // the value of the status variable and probably
            // give an error about mismatched types.
            *status = Status::InProgress;
        }
        Status::InProgress => {
            // Randomly choose between error and success.
            // This syntax (::<f32>), called turbofish,
            // is used to specify generic arguments.
            // Here, it's used to ask the random function to
            // return a 32-bit float.
            *status = if random::<f32>() < 0.5 {
                Status::Error(random())
            } else {
                // We need .into() to convert &str to String.
                Status::Success("Data Received!".into())
            };
        }
        _ => {}
    }
}

fn main() {
    let mut request = start_request();

    // Keep advancing the state until we're a success or an error.
    // matches! is another way to work with enums.
    // It returns true if the piece of data matches the specified pattern.
    while !matches!(request, Status::Success(_) | Status::Error(_)) {
        advance_request(&mut request);
        // Use debug printing (:?).
        println!("New Status: {request:?}");
    }

    // If request.into() (which uses our implementation above)
    // is a Some, then put the data inside it into a variable
    // called status.
    // Otherwise, return.
    let Some(status) = request.into() else {
        return;
    };

    println!("Unhandled Status: {status:?}");

    match status {
        Ok(msg) => println!("Success: {msg}"),
        Err(code) => println!("Failure: {code}"),
    }
}
