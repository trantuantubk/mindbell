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
use std::time::Duration;
use std::io;
use std::thread;
use notify_rust::Notification;

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

fn main() {
    // 1. Print the welcome message
    println!("==== Mindbell: Mindful work session timer\n");
    // Main loop: continue until user chooses to exit
    loop {
        // 2. Ask for the intent of this session
        // Create a new String to hold the intent
        let mut intent = String::new();
        println!("The intent of this session: ");
        io::stdin()
            .read_line(&mut intent)
            .expect("Failed to read input");
        // Remove whitespace
        intent = intent.trim().to_string();
        // If user just presses "Enter", then exit
        if intent.is_empty() {
            println!("May you be mindful. Goodbye.\n");
            break;
        }
        // 2. Ask for the duration in minutes (default: 25 mins)
        println!("The duration of this session in minutes (Enter for 25):\n");
        // Create a new String to hold the input duration
        let mut input_duration = String::new();
        io::stdin()
            .read_line(&mut input_duration)
            .expect("Failed to read duration");
        input_duration = input_duration.trim().to_string();
        let duration = input_duration.parse::<u32>().unwrap_or(25);
        // 4. Play the start session bell (TODO: add the real sound)
        println!("Starting bell, be present and mindful\n");
        print!("\x07");
        // Temporary: display the intent and duration
        println!("Intent: {intent}, duration: {duration} (min)\n");
        // 5. Sleep for duration minutes
        thread::sleep(Duration::from_secs((duration * 60) as u64));
        // 6. Play the end session bell (TODO: add the real sound)
        println!("Ending bell, Session Complete.\nTime to pause and Reflect!");
        print!("\x07");
        send_notification(
            "Session Complete",
            "Time to pause and reflect"
        );
        // 7. TODO: add the reflecting note (optional)
        // TODO: Ask for continuing the new session or not
        // print an empty line
        println!("");    
    }

    
}
