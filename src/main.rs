use quadratic_voting_rs::Session;
use std::io;

fn main() {
    let mut session = Session::create_session(100);
    //Using hard coded proposals, because the goal is to create a user facing application
    let _proposal_1 = session.add_proposal(
        "Scroll meetup".to_string(),
        "Organise a scroll meetup".to_string(),
    );

    let _proposal_2 = session.add_proposal(
        "Aztec dev workshop".to_string(),
        "Organise an aztec zk workshop for developers".to_string(),
    );

    let _proposal_3 = session.add_proposal(
        "Ethereum Layer 2 Discussion".to_string(),
        "Host a discussion about Ethereum Layer 2 scaling solutions".to_string(),
    );

    let _ = session.start_session();

    loop {
        println!("Enter 1 to register,\n Enter 2 to vote for a proposal,\n Enter 3 to See results, \n and Enter 4 to exit this appliction");

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Could not read line");
        let selection: i8 = match selection.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}\n", e);
                continue;
            }
        };

        match selection {
            1 => {
                println!("Enter you name");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Could not read name");

                // This is necessary to ensure that errors are properly handled and displayed
                match session.register_voter(name) {
                    Ok(_) => println!("User registerd sucessfully \n"),
                    Err(e) => println!("Error: {}\n", e),
                }
            }

            2 => {
                println!("Enter you voter id");
                let mut voter_id = String::new();
                io::stdin()
                    .read_line(&mut voter_id)
                    .expect("Could not read voter_id");

                let voter_id: i32 = match voter_id.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Error: {}\n", e);
                        continue;
                    }
                };

                println!("Enter the id of the proposal you want to vote for");
                let mut proposal_id = String::new();
                io::stdin()
                    .read_line(&mut proposal_id)
                    .expect("Could not read proposal_id");
                let proposal_id: i32 = match proposal_id.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Error: {}\n", e);
                        continue;
                    }
                };

                println!("Enter how many credits you want to vote with");
                let mut voter_credits = String::new();
                io::stdin()
                    .read_line(&mut voter_credits)
                    .expect("Could not read voter_credits");
                let voter_credits: i32 = match voter_credits.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Error: {}\n", e);
                        continue;
                    }
                };
                match session.vote(voter_id, proposal_id, voter_credits){
                    Ok(_) => println!("Voted sucessfully\n"),
                    Err(e) => println!("Error: {} \n", e)
                }  
            }

            3 => {
                session.get_results();
            }

            4 => {
                break;
            }

            _ => {
                println!("Invalid choice");
            }
        }
    }
}
