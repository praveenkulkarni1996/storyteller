// This is the storyteller game model.
use std::collections::VecDeque;

pub enum Name {
    Adam,
    Eve,
    Nobody,
}

pub enum Scene {
    Love { left: Name, right: Name },
    Death { grave: Name, witness: Name },
}

pub enum Event {
    InLove { me: Name, partner: Name },
    Dies { me: Name },
    IsHeartbroken { me: Name },
}

pub type Story = VecDeque<Scene>;
pub type Log = VecDeque<Event>;

fn main() {
    println!("hello world!");
}
