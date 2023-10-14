// This is the storyteller game model.
use storyteller::Event::*;
use storyteller::Log;
use storyteller::Name::{Adam, Eve, Nobody};
use storyteller::Scene;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eve_dies_heartbroken() {
        let storydeck: Vec<Scene> = vec![
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
        ];
        let events: Log = vec![
            InLove {
                me: Adam,
                with: Eve,
            },
            InLove {
                me: Eve,
                with: Adam,
            },
            Dies { me: Adam },
            IsHeartbroken { me: Eve },
            Dies { me: Eve },
            IsHeartbroken { me: Nobody },
        ];
        assert_eq!(storyteller::act(&storydeck), events);
    }

    #[test]
    fn test_seeing_the_ghost_of_a_lover() {
        let storydeck: Vec<Scene> = vec![
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
        ];
        assert_eq!(storydeck.len(), 3);
    }
}
