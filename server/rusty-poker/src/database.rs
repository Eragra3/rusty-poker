use crate::poker::{Vote, VoteValue, Voting};
use chrono::prelude::*;
use time::Duration;

pub trait PokerDatabase {
    fn set_vote_value(&mut self, voting_id: i32, vote_id: i32, value: VoteValue) -> Result<Vote, String>;

    fn get_votes(&self, voting_id: i32) -> Option<Vec<Vote>>;

    fn get_voting(&self, voting_id: i32) -> Option<Voting>;

    // fn create_voting() -> Result<Vote, ()> {
}

pub struct MockDatabase {
    votings: Vec<Voting>,
}

impl MockDatabase {
    pub fn new() -> MockDatabase {
        let mock_votings = vec![
            Voting::new(
                1,
                Utc::now() - Duration::minutes(10),
                vec![
                    Vote::new(1, VoteValue::Pending),
                    Vote::new(2, VoteValue::Coffee),
                    Vote::new(3, VoteValue::Pending),
                    Vote::new(4, VoteValue::Value(8)),
                    Vote::new(5, VoteValue::Value(13)),
                ],
            ),
            Voting::new(
                2,
                Utc::now() - Duration::minutes(9),
                vec![
                    Vote::new(6, VoteValue::Pending),
                    Vote::new(7, VoteValue::Pending),
                    Vote::new(8, VoteValue::Pending),
                    Vote::new(9, VoteValue::Pending),
                    Vote::new(10, VoteValue::Pending),
                ],
            ),
            Voting::new(
                3,
                Utc::now() - Duration::days(2),
                vec![
                    Vote::new(11, VoteValue::Value(8)),
                    Vote::new(12, VoteValue::Value(8)),
                    Vote::new(13, VoteValue::Value(5)),
                    Vote::new(14, VoteValue::Value(13)),
                    Vote::new(15, VoteValue::Value(8)),
                ],
            ),
            Voting::new(
                4,
                Utc::now() - Duration::days(1) - Duration::minutes(34),
                vec![
                    Vote::new(16, VoteValue::Coffee),
                    Vote::new(17, VoteValue::Coffee),
                    Vote::new(18, VoteValue::Coffee),
                    Vote::new(19, VoteValue::Value(21)),
                    Vote::new(20, VoteValue::Pending),
                ],
            ),
        ];
        // assert!(mock_votings.iter().map(|&x| x.id))
        MockDatabase {
            votings: mock_votings,
        }
    }
}

impl PokerDatabase for MockDatabase {
    fn set_vote_value(&mut self, voting_id: i32, vote_id: i32, value: VoteValue) -> Result<Vote, String> {
        let voting = self
            .votings
            .iter_mut()
            .find(|x| x.id == voting_id);

        let voting = match voting {
            Some(voting) => voting,
            _ => return Err(format!("Cannot find voting with id '{}'", voting_id))
        };

        let vote = voting
            .votes
            .iter_mut()
            .find(|x| x.id == vote_id);

        let vote = match vote {
            Some(vote) => vote,
            _ => return Err(format!("Cannot find vote with id '{}' inside voting '{}'", vote_id, voting_id))
        };

        vote.set_vote(value);

        Ok(vote.clone())
    }

    fn get_votes(&self, voting_id: i32) -> Option<Vec<Vote>> {
        let voting = self.votings.iter().find(|&x| x.id == voting_id);
        match voting {
            Some(voting) => Some(voting.votes.clone()),
            _ => None,
        }
    }

    fn get_voting(&self, voting_id: i32) -> Option<Voting> {
        self.votings.iter().find(|&x| x.id == voting_id).cloned()
    }
}
