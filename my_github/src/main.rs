extern crate rust_github;

use rust_github::Github;

fn main() {
    let github = Github::new();
    let user = github.users.get("sriharshakappala");
    let repositories = github.repositories.by_user("sriharshakappala");
    println!("Name: {:?}", user.name);
    println!("Email: {:?}", user.email);
    println!("Location: {:?}", user.location);
    for repo in repositories.iter() {
        println!("{:?}", repo.name);
    }
}
