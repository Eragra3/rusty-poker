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

    fn create_voting(&mut self, host_uuid: Uuid, title: String) -> Result<Voting, String>;

    fn create_user(&mut self, name: String) -> Result<User, String>;

    fn join_voting(&mut self, user_uuid: Uuid, voting_uuid: Uuid) -> Result<Vote, String>;
}

pub struct MockDatabase {
    votings: Vec<Voting>,
    users: Vec<User>,
}

impl MockDatabase {
    pub fn new() -> MockDatabase {
        let mock_users = vec![
            User::new(
                Uuid::parse_str("30bb18cb-d1b3-4225-afe9-ec10fa5d4c49").unwrap(),
                "Pan Paweł".to_string(),
                Utc::now() - Duration::minutes(110),
            ),
            User::new(
                Uuid::parse_str("021c5fd1-f63b-46fd-adfb-c3d05b1a0bc1").unwrap(),
                "Daniel".to_string(),
                Utc::now() - Duration::minutes(420),
            ),
            User::new(
                Uuid::parse_str("4a36252a-e282-4a72-bb2d-e04a9375e6b4").unwrap(),
                "Mathew".to_string(),
                Utc::now() - Duration::minutes(60),
            ),
            User::new(
                Uuid::parse_str("4223d56b-980d-401f-bab5-a78f37aa4bb4").unwrap(),
                "XxX_Xander_XxX".to_string(),
                Utc::now() - Duration::minutes(59),
            ),
            User::new(
                Uuid::parse_str("14301ee4-6563-40dd-8230-8f64d8c3c315").unwrap(),
                "Pat!1999".to_string(),
                Utc::now() - Duration::minutes(61),
            ),
            User::new(
                Uuid::parse_str("a9064385-ebc6-4790-a0b6-d75183d77481").unwrap(),
                "John ☺ ☻".to_string(),
                Utc::now() - Duration::minutes(1240),
            ),
            User::new(
                Uuid::parse_str("0cea304f-1778-4ce3-97fd-40584fc5315c").unwrap(),
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
                Uuid::parse_str("4452b3fe-b41b-47af-9e8e-e95a53c9da12").unwrap(),
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
                Uuid::parse_str("bbd5b87d-74ac-438a-98d8-c060d9201419").unwrap(),
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
                Uuid::parse_str("c577e70e-4730-4b20-800b-cf28027bc512").unwrap(),
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
                Uuid::parse_str("d9836573-f46d-4fe7-ad19-fd9432031869").unwrap(),
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

    fn create_voting(&mut self, host_uuid: Uuid, title: String) -> Result<Voting, String> {
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

    fn create_user(&mut self, name: String) -> Result<User, String> {
        let user = User::new(Uuid::new_v4(), name, Utc::now());

        self.users.push(user.clone());

        Ok(user)
    }

    fn join_voting(&mut self, user_uuid: Uuid, voting_uuid: Uuid) -> Result<Vote, String> {
        let _user = match self.users.iter().find(|x| x.uuid() == user_uuid) {
            Some(user) => user,
            None => return Err(format!("Cannot find user with uuid '{}'", user_uuid)),
        };

        let voting = match self.votings.iter_mut().find(|x| x.uuid() == voting_uuid) {
            Some(voting) => voting,
            None => return Err(format!("Cannot find voting with uuid '{}'", voting_uuid)),
        };

        let vote = match voting.votes().iter().find(|x| x.voter_uuid() == user_uuid) {
            Some(vote) => vote.clone(),
            None => {
                let vote = Vote::new(VoteValue::Pending, user_uuid);
                voting.votes_mut().push(vote.clone());
                vote
            }
        };

        Ok(vote)
    }
}
