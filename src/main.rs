static MENU_TEXT: &str = "\
KolkulatorRS, by HorseArmored Inc. (C)2024
Choose 1-9

1. Addition
2. Subtraction
3. Multiplicaion
4. Division
5. Power
6. Root
7. Quadratic Equasion
8. Real HDD/Flash space
9. Quit";

use kolkulator::short;

use std::io::{stdout, Write};
use crossterm::{terminal::{ClearType, Clear}, QueueableCommand, cursor::MoveTo, event::{read, Event, KeyCode}};

fn clear_screen() {
    let mut out = stdout();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}

fn key_eater() { // this bad boy somehow eats the ghost keys, so 1 key doesn't detect twice.
    while let Ok(event) = read().map(|event| event) {
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    _ => {
                        break;
                    }
                }
            }
            _ => panic!("Got a non key event while only expecting key events."),
        }
    }
}

fn get_input() -> String {
    key_eater();

    return read().map(|event| {
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char(c) => c.to_string(),
                    KeyCode::Enter => "Enter".to_string(),
                    KeyCode::Esc => "Esc".to_string(),
                    // Handle other key codes as needed
                    other => format!("{:?}", other),
                }
            }
            _ => panic!("Got a non key event while only expecting key events."),
        }
    }).unwrap();
}

fn press_any_key_to_continue() {
    println!("Press any key to continue...");
    
    key_eater();

    loop {
        if let Ok(event) = read().map(|event| event) {
            if let Event::Key(_) = event {
                break; // Exit loop on any key press event
            }
        }
    }
}

fn main() {
    loop {
        clear_screen();

        println!("{MENU_TEXT}");

        print!("#");
        stdout().flush().unwrap();
        let cmd = get_input();
        println!("{cmd}");

        match cmd.as_str() {
            "1" => short::addition(),
            "2" => short::subtraction(),
            "3" => short::multiplicaion(),
            "4" => short::division(),
            "5" => short::power(),
            "6" => short::root(),
            "7" => short::quadratic(),
            "8" => short::storage(),
            "9" => {
                println!("See you later aligator!");
                std::process::exit(0);
            },
            _ => println!("Options are 1-9 please respect that."),
        }
        press_any_key_to_continue()
    }
}
