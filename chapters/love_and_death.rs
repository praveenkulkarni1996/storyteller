// This is the storyteller game model.
use std::collections::VecDeque;
use storyteller::Name::{Adam, Eve, Nobody};
use storyteller::Scene;

#[cfg(test)]
mod tests {
    use super::*;

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
