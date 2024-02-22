mod repo;
mod commit;
mod staged;
mod serialize;

fn main() {

    let mut repo = repo::Repo::new();
    repo.init();
    repo.add("code/test1.txt".to_string());
    repo.add("code/test2.txt".to_string());

    repo.commit("initial commit".to_string());
    println!("staged are: {:?}", repo.get_staged());


    println!("Hello, world!");

}



