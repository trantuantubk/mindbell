// Mindbell: a mindful task management inspired by the Pomodoro clock and Vipassana meditation practice
// Each session starts with a gentle bell and end with a gentle bell
// During the session, it is the silent for awareness and mindfulness, no ticking clock, no countdown, just presence
// No rush for result or deadline, just focusing on the current work
// By default, the duration of each session is 25 minutes
// The user is given a contemplative moment after each session to take notes,
// or simply rest in contemplation merging with eternal tranquility

// Step 1: The main loop
// Implement:
// 1. Print the welcome message
// 2. Ask for the intent of this session
// 3. Ask for the duration in minute (default 25 mins) 
// 4. Play the start session bell
// 5. Sleeping in duration
// 6. Play the end session bell
// 7. Ask for note in the contemplating session
// Return to 2

// lib.rs: core logic for testablility.
use mindbell::{get_input, parse_duration};
use std::time::Duration;
use std::io::Write;
use std::fs::OpenOptions;
use std::thread;
use notify_rust::Notification;
use chrono::Local; // import local time


// Send the notification as a popup dialog
// Just to remind the user that the session ends
fn send_notification(title: &str, message: &str) {
    // a failed notificaton shouldn't stop the program
    // it is not critical
    if let Err(e) = Notification::new()
        .summary(title)
        .body(message)
        .timeout(0)
        .show()
    {
         eprintln!("Counld not send notification: {e}");
    }
}

fn main() -> std::io::Result<()> {
    // Configuration constants
    const DEFAULT_DURATION: u32 = 25;

    // Initialization
    let reflecting_note_filename = "mindnote.txt";
    // 1. Print the welcome message
    println!("==== Mindbell: Mindful work session timer\n");
    // Main loop: continue until user chooses to exit
    loop {
        // 2. Ask for the intent of this session
        let intent = get_input("The intent of this session: ");
        // If the intent is empty, then exit
        if intent.is_empty() {
            println!("May you be mindful. Goodbye.\n");
            break;
        }
        // 2. Ask for the duration in minutes (default: 25 mins)
        let duration_prompt = format!("The duration of this session in minutes (Enter for {DEFAULT_DURATION} mins): ");
        let duration_input = get_input(&duration_prompt);
        // Create a new String to hold the input duration
        let duration = parse_duration(&duration_input, DEFAULT_DURATION);
        // 4. Play the start session bell (TODO: add the real sound)
        println!("Starting bell, be present and mindful\n");
        print!("\x07");
        // Display the intent and duration
        println!("Intent: {intent}, duration: {duration} (min)\n");
        // Get the current local time to write the intent and duration
        let start_timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        // 5. Sleep for duration minutes
        thread::sleep(Duration::from_secs((duration * 60) as u64));
        // 6. Play the end session bell (TODO: add the real sound)
        println!("Ending bell, Session Complete.\nTime to pause and Reflect!");
        print!("\x07");
        send_notification(
            "Session Complete",
            "Time to pause and reflect"
        );
        // 7. Add the reflecting note (optional)
        // Ask for the reflecting note of this session
        let reflecting_note = get_input("Reflecting note (optional): ");
        // Record the local time for reflecting note
        let end_timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        // Open the reflecting note file, to save the reflecting node
        // If the file doesn't exist: create new, otherwise: open to append
        // Place the file open and write within the loop, 
        // to allow the file is opened and automatically closed when go out of the block scope
        {
            let mut reflecting_note_file = OpenOptions::new()
                .append(true) // Opens in append mode
                .create(true) // Creates file if it does not exist
                .open(reflecting_note_filename)?; // Atomic operation
            // Append the intent and duration to the file
            writeln!(reflecting_note_file, "[{}] Intent {}, duration {} (min)", start_timestamp, intent, duration)?;
            // Append the reflecting note to the file
            writeln!(reflecting_note_file, "[{}] {}", end_timestamp, reflecting_note)?;        
        }
        // Display the new line to the terminal, to start the new loop 
        println!("");    
    }
    Ok(())
    
}
