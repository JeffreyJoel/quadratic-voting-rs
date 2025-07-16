use quadratic_voting_rs::*;

/// Happy paths
#[test]
fn test_create_session() {
    let session = Session::create_session(100);
    assert_eq!(session.total_credits, 100);
    assert_eq!(session.voters.len(), 0);
    assert_eq!(session.votes.len(), 0);
    assert_eq!(session.proposals.len(), 0);
    assert_eq!(session.is_active, false);
}

#[test]
fn test_start_session(){
    let mut session = Session::create_session(100);
    assert_eq!(session.is_active, false);

    let session_started = session.start_session();
    assert_eq!(session_started.is_ok(), true);
    assert_eq!(session.is_active, true);
}

#[test]
fn test_add_proposal() {
    let mut session = Session::create_session(100);
    let result = session.add_proposal(
        "Scroll meetup".to_string(),
        "Organise a scroll meetup".to_string(),
    );
    assert_eq!(result.is_ok(), true);
    assert_eq!(session.proposals.len(), 1);
    assert_eq!(session.proposals[0].title, "Scroll meetup");
    assert_eq!(session.proposals[0].description, "Organise a scroll meetup");
    assert_eq!(session.proposals[0].total_votes, 0.0);
}

#[test]
fn test_register_voter(){
    let mut session = Session::create_session(100);

    let voter_created_1 = session.register_voter("Joe".to_string());

    assert_eq!(voter_created_1.is_ok(), true);
    assert_eq!(session.next_voter_id, 1);

    let registered_voter = session.voters.get(&session.next_voter_id).unwrap();
    assert_eq!(registered_voter.name, "Joe");

    let voter_created_2 = session.register_voter("Jane".to_string());
    assert_eq!(voter_created_2.is_ok(), true);
    assert_eq!(session.next_voter_id, 2);

    let registered_voter_2 = session.voters.get(&session.next_voter_id).unwrap();
    assert_eq!(registered_voter_2.name, "Jane");
}

#[test]
fn test_vote(){
    let mut session = Session::create_session(100);
    let  _proposal_1 = session.add_proposal(
        "Scroll meetup".to_string(),
        "Organise a scroll meetup".to_string(),
    );

    let _proposal_2 = session.add_proposal(
        "Aztec dev workshop".to_string(),
        "Organise an aztec zk workshop for developers".to_string(),
    );

    let  _voter_created_1 = session.register_voter("Joe".to_string());
    let proposal_id:i32 = 1;
    let  vote = session.vote(1, proposal_id, 36);

    assert_eq!(vote.is_ok(), true);
    // assert_eq!(session.get_voter_votes(1, proposal_id), 6.0);
    // assert_eq!(*session.proposals[0].user_voted.get(&1).unwrap(), true);
    // assert_eq!(session.voters.get(&1).unwrap().credits, 64 )
}

//TODO: add un-happy paths

