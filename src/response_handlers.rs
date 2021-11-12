use std::thread;

use chrono::{DateTime, FixedOffset, Utc};

// A function that takes one argument 'timestamp'. This should be a &str in the form of an ISO
// 8601 UTC timestamp. The function calculates the duration between UTC now and the timestamp, and
// makes the thread sleep until the time has elapsed.
pub fn sleep(timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {

    // The current UTC time.
    let utc_now: DateTime<Utc> = Utc::now();

    // Parse the timestamp String into a chrono::DateTime::<FixedOffset>.
    let chrono_ts: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp)?;

    // Calculate the duration, as a chrono::OldDuration.
    let dur = chrono_ts.signed_duration_since(utc_now);

    // Convert the chrono::OldDuration to a std::time::Duration.
    let std_dur = dur.to_std()?;

    println!("Putting thread to sleep for {} seconds.", &std_dur.as_secs());
    thread::sleep(std_dur);

    Ok(())
}