// This is the storyteller game model.
use std::collections::VecDeque;

enum Name {
    Adam,
    Eve,
    Nobody,
}

enum Scene {
    Love { left: Name, right: Name },
    Death { grave: Name, witness: Name },
}

enum Event {
    InLove { me: Name, partner: Name },
    Dies { me: Name },
    IsHeartbroken { me: Name },
}

type Story = VecDeque<Scene>;
type Log = VecDeque<Event>;

fn main() {
    println!("hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Name::*;

    #[test]
    fn test_eve_dies_heartbroken() {
        let storydeck: VecDeque<Scene> = VecDeque::from(vec![
            Scene::Love {
                left: Adam,
                right: Eve,
            },
            Scene::Death {
                grave: Adam,
                witness: Eve,
            },
            Scene::Death {
                grave: Eve,
                witness: Nobody,
            },
        ]);
        assert_eq!(storydeck.len(), 3);
    }

    #[test]
    fn test_seeing_the_ghost_of_a_lover() {
        let storydeck: VecDeque<Scene> = VecDeque::from(vec![
            Scene::Love {
                left: Adam,
                right: Eve,
            },
            Scene::Death {
                grave: Adam,
                witness: Eve,
            },
            Scene::Love {
                left: Adam,
                right: Eve,
            },
        ]);
        assert_eq!(storydeck.len(), 3);
    }
}
