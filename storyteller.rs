#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Name {
    Adam,
    Eve,
    Nobody,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Scene {
    Love { left: Name, right: Name },
    Death { grave: Name, witness: Name },
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Event {
    InLove { me: Name, partner: Name },
    Dies { me: Name },
    IsHeartbroken { me: Name },
}

pub type Story = Vec<Scene>;
pub type Log = Vec<Event>;

fn enact(scene: Scene, log: Log) -> Log {
    use crate::Event::*;
    use crate::Name::*;
    use crate::Scene::*;
    let events = match scene {
        Love { left, right } => vec![
            InLove {
                me: left,
                partner: right,
            },
            InLove {
                me: right,
                partner: left,
            },
        ],
        Death { grave, witness } => vec![Dies { me: grave }, IsHeartbroken { me: witness }],
    };
    dbg!(&events);
    return events;
}

pub fn act(scenes: &[Scene]) {
    for scene in scenes {
        enact(*scene, vec![]);
    }
}

fn main() {
    println!("hello world!");
}
