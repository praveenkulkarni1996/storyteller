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
    InLove { me: Name, with: Name },
    Dies { me: Name },
    IsHeartbroken { me: Name },
    IsScaredByGhost { me: Name, ghost: Name },
}

pub type Story = Vec<Scene>;
pub type Log = Vec<Event>;

pub fn is_alive(eventlog: &Log, name: Name) -> bool {
    for event in eventlog.iter().rev() {
        match *event {
            Event::Dies { me } if me == name => return false,
            _ => {}
        };
    }
    return true;
}

pub fn is_heartbroken(eventlog: &Log, name: Name) -> bool {
    for event in eventlog.iter().rev() {
        match *event {
            Event::IsHeartbroken { me } if me == name => return true,
            Event::InLove { me, .. } if me == name => return false,
            _ => {}
        };
    }
    return false;
}

fn enact(scene: Scene, log: &Log) -> Log {
    use crate::Event::*;
    use crate::Name::*;
    use crate::Scene::*;
    let events = match scene {
        Love { left, right } if is_alive(log, left) && is_alive(log, right) => vec![
            InLove {
                me: left,
                with: right,
            },
            InLove {
                me: right,
                with: left,
            },
        ],
        Love { left, right } if is_alive(log, left) && !is_alive(log, right) => {
            vec![IsScaredByGhost {
                me: left,
                ghost: right,
            }]
        }
        Love { left, right } if !is_alive(log, left) && is_alive(log, right) => {
            vec![IsScaredByGhost {
                me: right,
                ghost: left,
            }]
        }
        Love { .. } => vec![],
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
