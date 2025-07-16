use std::collections::HashMap;

#[derive(Debug)]
pub struct Session {
    pub total_credits: i32,
    pub proposals: Vec<Proposal>,
    pub voters: HashMap<i32, Voter>,
    pub next_voter_id: i32,
    pub votes: HashMap<(i32, usize), Vote>,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct Proposal {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub total_votes: f64,
    pub user_voted: HashMap<i32, bool>
}

#[derive(Debug)]
pub struct Voter {
    pub id: i32,
    pub name: String,
    pub credits: i32,
}

#[derive(Debug)]
pub struct Vote {
    pub voter_id: i32,
    pub proposal_id: usize,
    pub credits_spent: i32,
    pub vote_count: f64,
}

impl Session {
    pub fn create_session(total_credits: i32) -> Self {
        Self {
            total_credits,
            proposals: vec![],
            voters: HashMap::new(),
            next_voter_id: 0,
            votes: HashMap::new(),
            is_active: false,
        }
    }

    pub fn start_session(&mut self) -> Result<(), String>{
        if self.is_active{
            return Err("Session is already active".to_string());
        }
        self.is_active = true;
        Ok(())
    }

    pub fn add_proposal(&mut self, title: String, description: String) -> Result<(), String> {
        let proposal_id = self.proposals.len();

        if self.is_active {
            return Err("Cannot add proposal after session has started".to_string());
        }

        if title.is_empty() || description.is_empty() {
            return Err("Title and description cannot be empty".to_string());
        }

        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            total_votes: 0.0,
            user_voted: HashMap::new(),
        };
        self.proposals.push(proposal);
        Ok(())
    }

    pub fn register_voter(&mut self, name: String) -> Result<(), String> {
        self.next_voter_id += 1;
        let voter_id = self.next_voter_id;

        if self.is_active {
            return Err("Session is already active.".to_string());
        }

        if name.is_empty() {
            return Err("Voter name cannot be empty".to_string());
        }

        if self.voters.contains_key(&voter_id) {
            return Err("Voter already exists".to_string());
        }

        let voter = Voter {
            id: voter_id,
            name,
            credits: self.total_credits,
        };
        self.voters.insert(voter_id, voter);
        Ok(())
    }

    pub fn vote(
        &mut self,
        voter_id: i32,
        proposal_id: usize,
        vote_credits: i32,
    ) -> Result<(), String> {
        if !self.voters.contains_key(&voter_id) {
            return Err("You are not a voter".to_string());
        }

        if self.voters.get(&voter_id).unwrap().credits < vote_credits {
            return Err("You don't have enough credits to vote".to_string());
        }
        if vote_credits <= 0 {
            return Err("You cannot vote with zero credits".to_string());
        }

        self.voters.get_mut(&voter_id).unwrap().credits -= vote_credits;

        self.proposals[proposal_id].user_voted.insert(voter_id, true);

        let vote_count = (vote_credits as f64).sqrt();
        self.proposals[proposal_id].total_votes += vote_count;
        
        let vote_key = (voter_id, proposal_id);
        let vote = Vote {
            voter_id,
            proposal_id,
            credits_spent: vote_credits,
            vote_count: vote_count,
        };

        self.votes.insert(vote_key, vote);
        Ok(())
    }

    pub fn get_voter_votes(&self, voter_id: i32, proposal_id: usize) -> f64 {
        self.votes
            .get(&(voter_id, proposal_id))
            .map(|v| v.vote_count)
            .unwrap()
    }

    pub fn get_voter_total_credits_spent(&self, voter_id: i32) -> i32 {
        self.votes
            .values()
            .filter(|v| v.voter_id == voter_id)
            .map(|v| v.credits_spent)
            .sum()
    }

    pub fn get_results(&self) {
        for proposal in &self.proposals {
            println!(
                "Proposal: {title} | Votes: {total_votes:.1}",
                title = proposal.title,
                total_votes = proposal.total_votes,
            );

            let mut proposal_votes: Vec<_> = self
                .votes
                .values()
                .filter(|v| v.proposal_id == proposal.id)
                .collect();
            proposal_votes.sort_by_key(|v| v.voter_id);

            for vote in proposal_votes {
                let voter_name = &self.voters.get(&vote.voter_id).unwrap().name;
                let vote_count = (vote.credits_spent as f64).sqrt();
                println!(
                    "  → {}: {} votes ({} credits)",
                    voter_name, vote_count as i32, vote.credits_spent
                );
            }
            println!();
        }
    }
}
