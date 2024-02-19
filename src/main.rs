mod repo;
mod commit;
mod staged;
mod serialize;

fn main() {

    let mut repo = repo::Repo::new();
    repo.init();
    repo.add("test1.txt".to_string());

    println!("staged are: {:?}", repo.get_staged());


    println!("Hello, world!");

}



