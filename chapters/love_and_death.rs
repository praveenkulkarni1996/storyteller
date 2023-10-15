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
        return Status::Err(String::from("Eve is not dead"));
    }
    // Eve must be heartbroken.
    if !storyteller::is_heartbroken(&events, Eve) {
        return Status::Err(String::from("Eve is not heartbroken"));
    }
    // TODO: Story must only use the scenes `Love` and `Death`.
    // TODO: Story must only use the characters `Adam` and `Eve`.
    return Status::Ok;
}

fn seeing_the_ghost_of_a_lover(story: Story) -> Status {
    let events: Log = storyteller::act(&story);
    if story.len() > 3 {
        return Status::Err(String::from("story has more than 3 scenes."));
    }
    unimplemented!();
    // TODO: Story must only use the scenes `Love` and `Death`.
    // TODO: Story must only use the characters `Adam` and `Eve`.
    // TODO: Name must have IsScaredByGhost in their history.
    // TODO: Name must have be InLove with the ghost before this point.
}

#[cfg(test)]
mod eve_dies_heartbroken_tests {
    use super::*;

    #[test]
    fn test_win() {
        // adam and eve fall in love.
        // adam dies and eve watches.
        // eve dies.
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
    fn test_no_heartbreak() {
        // adam and eve fall in love.
        // eve dies.
        let storydeck: Vec<Scene> = vec![
            Scene::Love {
                left: Adam,
                right: Eve,
            },
            Scene::Death {
                grave: Eve,
                witness: Nobody,
            },
        ];
        assert_eq!(
            eve_dies_heartbroken(storydeck),
            Status::Err(String::from("Eve is not heartbroken"))
        );
    }

    #[test]
    fn test_no_death() {
        // adam and eve fall in love.
        // adam dies.
        let storydeck: Vec<Scene> = vec![
            Scene::Love {
                left: Adam,
                right: Eve,
            },
            Scene::Death {
                grave: Adam,
                witness: Nobody,
            },
        ];
        assert_eq!(
            eve_dies_heartbroken(storydeck),
            Status::Err(String::from("Eve is not dead"))
        );
    }
}

#[cfg(test)]
mod seeing_the_ghost_of_a_lover_tests {
    use super::*;

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
