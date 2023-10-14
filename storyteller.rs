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

fn is_alive(eventlog: &Log, name: Name) -> bool {
    for event in eventlog.iter().rev() {
        if *event == (Event::Dies { me: name }) {
            return false;
        }
    }
    return true;
}

fn enact(scene: Scene, log: &Log) -> Log {
    use crate::Event::*;
    use crate::Name::*;
    use crate::Scene::*;
    let events = match scene {
        Love { left, right } => {
            if is_alive(log, left) && is_alive(log, right) {
                vec![
                    InLove {
                        me: left,
                        partner: right,
                    },
                    InLove {
                        me: right,
                        partner: left,
                    },
                ]
            } else {
                vec![]
            }
        }
        Death { grave, witness } => vec![Dies { me: grave }, IsHeartbroken { me: witness }],
    };
    return events;
}

pub fn act(scenes: &[Scene]) -> Log {
    let mut log: Log = Vec::new();
    for scene in scenes {
        let actions: Log = enact(*scene, &log);
        log.extend(actions);
    }
    dbg!(&log);
    return log;
}

fn main() {
    println!("hello world!");
}
