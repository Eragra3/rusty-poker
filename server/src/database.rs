use crate::poker::{Vote, VoteValue, Voting};

pub struct MockDatabase {
    votings: Vec<Voting>,
}

impl MockDatabase {
    pub fn new() -> MockDatabase {
        let mock_votings = vec![
            Voting::new(
                1,
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

    pub fn get_votes(&self, voting_id: i32) -> Option<&Vec<Vote>> {
        let voting = self.votings.iter().find(|&x| x.id == voting_id);
        match voting {
            Some(voting) => Some(&voting.votes),
            _ => None,
        }
    }
}
