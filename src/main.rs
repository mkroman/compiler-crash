extern crate irc;

use irc::client::data::command::Command;

pub enum Event<'a> {
    Command(&'a str, Vec<&'a str>)
}

struct Plugin;

impl Plugin {
    fn process(&self, command: Command, event: Event) {
        match event {
            Event::Command(ref cmd, ref args) => {
                let Command::PRIVMSG(ref target, ref msg) = command; // <--
            }
        }
    }
}

fn main() {
    let plugin = Plugin;
    let event = Event::Command("hello", vec!["world"]);
    let cmd = Command::PRIVMSG("foo".to_string(), "bar".to_string());

    plugin.process(cmd, event);
}

