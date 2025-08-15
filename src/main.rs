use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;

struct Question<'a> {
    text: &'a str,
    options: [&'a str; 3],
    correct: char,
}

fn type_out(text: &str, delay_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

fn rainbow_wave(text: &str, delay_ms: u64) {
    let colors = ["\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[36m", "\x1b[35m"];
    for (i, c) in text.chars().enumerate() {
        let color = colors[i % colors.len()];
        print!("{}{}", color, c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    print!("\x1b[0m\n");
}

fn animate_question(text: &str, delay_ms: u64, frames: usize) {
    for frame in 0..frames {
        print!("\r");
        for (i, c) in text.chars().enumerate() {
            if c.is_alphabetic() {
                let wave = (((i as f32 + frame as f32) * 0.5).sin()) > 0.0;
                if wave { print!("{}", c.to_ascii_uppercase()); }
                else { print!("{}", c.to_ascii_lowercase()); }
            } else { print!("{}", c); }
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

fn curtain_intro() {
    let width = 50;
    let curtain_char = '█';
    let empty = ' ';
    let frames = width / 2;

    for i in 0..=frames {
        print!("\x1B[2J\x1B[1;1H"); // clear screen

        for _ in 0..5 {
            let left = curtain_char.to_string().repeat(frames - i);
            let middle = empty.to_string().repeat(i * 2);
            let right = curtain_char.to_string().repeat(frames - i);
            println!("{}{}{}", left, middle, right);
        }

        // Stage with moving spotlight
        let spotlight_pos = (i * 2).min(width - 5);
        let mut stage_line = String::new();
        for j in 0..width {
            if j >= spotlight_pos && j < spotlight_pos + 5 {
                // push single char
                stage_line.push("🎬✨🎥✨🎬".chars().nth(j - spotlight_pos).unwrap_or(' '));
            } else {
                stage_line.push('─');
            }
        }
        println!("{}", stage_line);

        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
}

fn ending_ascii_art() {
    let art = r#"
      █████████████████████████████
      █                           █
      █        THE END 🎬         █
      █                           █
      █████████████████████████████
    "#;
    println!("{}", art);
    sleep(Duration::from_millis(2000));
}

fn main() {
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let cyan = "\x1b[36m";
    let yellow = "\x1b[33m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    curtain_intro();

    println!("\n");
    rainbow_wave("🎬  A24 QUIZ GAME  🎬", 25);
    println!("\nTest your A24 knowledge and claim your crown...\n");
    sleep(Duration::from_millis(400));

    let mut questions = vec![
        Question {
            text: "Which A24 movie released first?",
            options: ["Midsommar", "Hereditary", "Saint Maud"],
            correct: 'B',
        },
        Question {
            text: "Which A24 movie had the largest production budget?",
            options: ["Civil War", "Death of a Unicorn", "Heretic"],
            correct: 'A',
        },
        Question {
            text: "Which is A24's highest-grossing film worldwide?",
            options: ["The Whale", "Everything Everywhere All At Once", "Civil War"],
            correct: 'B',
        },
    ];

    let mut rng = rand::thread_rng();
    questions.shuffle(&mut rng);

    let mut score = 0;
    let mut answer = String::new();
    let start = Instant::now();

    for (i, q) in questions.iter().enumerate() {
        let mut opts = q.options.clone();
        opts.shuffle(&mut rng);
        let letters = ['A', 'B', 'C'];
        let correct_letter = letters[opts.iter().position(|&x| x == q.options[(q.correct as u8 - b'A') as usize]).unwrap()];

        print!("{yellow}Question {}/{}: {reset}", i + 1, questions.len());
        animate_question(q.text, 30, 15);

        for (l, o) in letters.iter().zip(opts.iter()) {
            type_out(&format!("{}{}. {}{}", cyan, l, o, reset), 15);
        }

        print!("{yellow}Your answer: {reset}");
        io::stdout().flush().unwrap();
        answer.clear();
        io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().eq_ignore_ascii_case(&correct_letter.to_string()) {
            println!("{green}✔ Correct! 🎬{reset}\n");
            score += 1;
        } else {
            println!("{red}✘ Incorrect. The correct answer is {}.{reset}\n", correct_letter);
        }
    }

    let duration = start.elapsed();
    let percentage = (score as f32 / questions.len() as f32) * 100.0;

    println!("{bold}{yellow}===================================================={reset}");
    type_out(&format!("You scored {} out of {} ({:.2}%) 🏆", score, questions.len(), percentage), 50);
    type_out(&format!("Time taken: {:.2?}", duration), 30);
    println!("{bold}{yellow}===================================================={reset}\n");

    ending_ascii_art();
}

