use crate::poker::{User, Vote, VoteValue, Voting, VotingState};
use chrono::prelude::*;
use time::Duration;
use uuid::Uuid;

pub trait PokerDatabase {
    fn set_vote_value(
        &mut self,
        voting_uuid: Uuid,
        voter_id: Uuid,
        value: VoteValue,
    ) -> Result<Vote, String>;

    fn get_votes(&self, voting_uuid: Uuid) -> Option<Vec<Vote>>;

    fn get_voting(&self, voting_uuid: Uuid) -> Option<Voting>;

    fn create_voting(&mut self, host_uuid: Uuid, title: String) -> Result<Voting, ()>;
}

pub struct MockDatabase {
    votings: Vec<Voting>,
    users: Vec<User>,
}

impl MockDatabase {
    pub fn new() -> MockDatabase {
        let mock_users = vec![
            User::new(
                Uuid::new_v4(),
                "Pan Paweł".to_string(),
                Utc::now() - Duration::minutes(110),
            ),
            User::new(
                Uuid::new_v4(),
                "Daniel".to_string(),
                Utc::now() - Duration::minutes(420),
            ),
            User::new(
                Uuid::new_v4(),
                "Mathew".to_string(),
                Utc::now() - Duration::minutes(60),
            ),
            User::new(
                Uuid::new_v4(),
                "XxX_Xander_XxX".to_string(),
                Utc::now() - Duration::minutes(59),
            ),
            User::new(
                Uuid::new_v4(),
                "Pat!1999".to_string(),
                Utc::now() - Duration::minutes(61),
            ),
            User::new(
                Uuid::new_v4(),
                "John ☺ ☻".to_string(),
                Utc::now() - Duration::minutes(1240),
            ),
            User::new(
                Uuid::new_v4(),
                "Some guy".to_string(),
                Utc::now() - Duration::minutes(5236),
            ),
        ];

        let mock_votings = vec![
            Voting::new(
                Utc::now() - Duration::minutes(10),
                vec![
                    Vote::new(VoteValue::Pending, mock_users[0].uuid()),
                    Vote::new(VoteValue::Coffee, mock_users[1].uuid()),
                    Vote::new(VoteValue::Pending, mock_users[2].uuid()),
                    Vote::new(VoteValue::Value(8), mock_users[3].uuid()),
                    Vote::new(VoteValue::Value(13), mock_users[4].uuid()),
                ],
                "Create new log in page".to_string(),
                VotingState::InProgress,
                Uuid::new_v4(),
                mock_users[3].uuid(),
            ),
            Voting::new(
                Utc::now() - Duration::minutes(9),
                vec![
                    Vote::new(VoteValue::Pending, mock_users[0].uuid()),
                    Vote::new(VoteValue::Pending, mock_users[1].uuid()),
                    Vote::new(VoteValue::Pending, mock_users[2].uuid()),
                    Vote::new(VoteValue::Pending, mock_users[3].uuid()),
                    Vote::new(VoteValue::Pending, mock_users[4].uuid()),
                ],
                "Rework security implementation".to_string(),
                VotingState::InProgress,
                Uuid::new_v4(),
                mock_users[0].uuid(),
            ),
            Voting::new(
                Utc::now() - Duration::days(2),
                vec![
                    Vote::new(VoteValue::Value(8), mock_users[2].uuid()),
                    Vote::new(VoteValue::Value(8), mock_users[3].uuid()),
                    Vote::new(VoteValue::Value(5), mock_users[4].uuid()),
                    Vote::new(VoteValue::Value(13), mock_users[5].uuid()),
                    Vote::new(VoteValue::Value(8), mock_users[6].uuid()),
                ],
                "Update database to new major version".to_string(),
                VotingState::Closed,
                Uuid::new_v4(),
                mock_users[2].uuid(),
            ),
            Voting::new(
                Utc::now() - Duration::days(1) - Duration::minutes(34),
                vec![
                    Vote::new(VoteValue::Coffee, mock_users[4].uuid()),
                    Vote::new(VoteValue::Coffee, mock_users[1].uuid()),
                    Vote::new(VoteValue::Coffee, mock_users[3].uuid()),
                    Vote::new(VoteValue::Value(21), mock_users[0].uuid()),
                    Vote::new(VoteValue::Pending, mock_users[2].uuid()),
                ],
                "Remove all code smells".to_string(),
                VotingState::InProgress,
                Uuid::new_v4(),
                mock_users[4].uuid(),
            ),
        ];
        // assert!(mock_votings.iter().map(|&x| x.id))
        MockDatabase {
            votings: mock_votings,
            users: mock_users,
        }
    }
}

impl PokerDatabase for MockDatabase {
    fn set_vote_value(
        &mut self,
        voting_uuid: Uuid,
        voter_uuid: Uuid,
        value: VoteValue,
    ) -> Result<Vote, String> {
        let voting = self.votings.iter_mut().find(|x| x.uuid() == voting_uuid);

        let voting = match voting {
            Some(voting) => voting,
            _ => return Err(format!("Cannot find voting with uuid '{}'", voting_uuid)),
        };

        let vote = voting
            .votes_mut()
            .iter_mut()
            .find(|x| x.voter_uuid() == voter_uuid);

        let vote = match vote {
            Some(vote) => vote,
            _ => {
                return Err(format!(
                    "Cannot find vote for user with uuid '{}' inside voting '{}'",
                    voter_uuid, voting_uuid
                ))
            }
        };

        vote.set_vote(value);

        Ok(vote.clone())
    }

    fn get_votes(&self, voting_id: Uuid) -> Option<Vec<Vote>> {
        let voting = self.votings.iter().find(|&x| x.uuid() == voting_id);
        match voting {
            Some(voting) => Some(voting.votes().clone()),
            _ => None,
        }
    }

    fn get_voting(&self, voting_id: Uuid) -> Option<Voting> {
        self.votings
            .iter()
            .find(|&x| x.uuid() == voting_id)
            .cloned()
    }

    fn create_voting(&mut self, host_uuid: Uuid, title: String) -> Result<Voting, ()> {
        let voting = Voting::new(
            Utc::now(),
            Vec::<Vote>::new(),
            title,
            VotingState::Waiting,
            Uuid::new_v4(),
            host_uuid,
        );

        self.votings.push(voting.clone());

        Ok(voting)
    }
}
