use ansi_term::Color;
pub struct Message;

impl Message {
    pub fn error(name: &str, message: String) -> anyhow::Result<()> {
        if !message.is_empty() {
            if !name.is_empty() {
                println!("{}: ", Color::White.paint(name));
            }
            println!("{}", Color::Red.paint(message));
        }

        Ok(())
    }

    pub fn normal(name: &str, messages: Vec<String>) -> anyhow::Result<()> {
        if !messages.is_empty() {
            if !name.is_empty() {
                println!("{}: ", Color::White.paint(name));
            }
            for message in messages {
                println!("{}", Color::Blue.paint(message));
            }
        }

        Ok(())
    }

    // pub fn normal(name: &str, messages: Vec<String>) -> anyhow::Result<()> {
    //     if !messages.is_empty() {
    //         if !name.is_empty() {
    //             for message in messages {
    //                 for line in message.lines() {
    //                     println!(
    //                         "{} {}",
    //                         Color::Blue.paint(format!("[{}]", name)),
    //                         Color::White.paint(line)
    //                     );
    //                 }
    //             }
    //         } else {
    //             for message in messages {
    //                 println!("{}", Color::White.paint(message));
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    pub fn info(message: String) -> anyhow::Result<()> {
        if !message.is_empty() {
            println!("{}", Color::White.paint(message));
        }

        Ok(())
    }
}
