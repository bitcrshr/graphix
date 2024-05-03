use graphix::Entity;
use graphix::entity::Entity;

#[derive(Entity)]
struct User {
    #[graphix(colname = "user_id")]
    #[graphix(immutable)]
    id: String
}

fn main() {
    let e = User { id: String::from("foo") };

    println!("Got user!\n\n{:?}", e.entity_data())
}
