// This is the storyteller game model.
use storyteller::Log;
use storyteller::Name::{Adam, Eve, Nobody};
use storyteller::Scene;
use storyteller::Story;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Status {
    Ok,
    Err(String),
}

fn eve_dies_heartbroken(story: Story) -> Status {
    let events: Log = storyteller::act(&story);
    // Story must only have 3 scenes or fewer.
    if story.len() > 3 {
        return Status::Err(String::from("story has more than 3 scenes."));
    }
    // Eve must be dead.
    if storyteller::is_alive(&events, Eve) {
        return Status::Err(String::from("Eve is still alive"));
    }
    // Eve must be heartbroken.
    if !storyteller::is_heartbroken(&events, Eve) {
        return Status::Err(String::from("eve is not heartbroken"));
    }
    // TODO: Story must only use the scenes `Love` and `Death`.
    // TODO: Story must only use the characters `Adam` and `Eve`.
    return Status::Ok;
}

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
        assert_eq!(eve_dies_heartbroken(storydeck), Status::Ok);
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
