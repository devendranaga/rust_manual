struct Person {
    name : String,
    age : u32,
}

struct VoterData {
    person : Person,
    is_eligible : bool,
}

fn main() {
    let name = String::from("person");
    let age = 32;

    /* Initialize structure */
    let p1 = Person {
        name: "Dev".to_string(),
        age: 33
    };

    let p2 = Person { name , age };

    println!("name: {} age: {}", p1.name, p1.age);
    println!("name: {} age: {}", p2.name, p2.age);

    let voter1 = VoterData {
        person : p1,
        is_eligible : true,
    };

    let voter2 = VoterData {
        person : p2,
        is_eligible : true,
    };

    let mut voters : Vec<VoterData> = Vec::new();

    voters.push(voter1);
    voters.push(voter2);

    for voter in voters {
        println!("person : [ name: {}, age: {} ], is_eligible: {}",
                    voter.person.name, voter.person.age,
                    voter.is_eligible);
    }
}
