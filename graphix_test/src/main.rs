
use uuid::Uuid;
use graphix::Entity;
use graphix::entity::Entity;

#[derive(Entity)]
#[graphix(table_name = "user", schema_name = "schema.private")]
struct User {
    #[graphix(colname = "user_id", unique)]
    pub id: String,
    pub name: String,
    pub username: String,
    pub verified: bool,
    #[graphix(nullable)]
    pub created_at: u64,
}

fn main() {
    let u = User {
        id: Uuid::new_v4().to_string(),
        name: "John Doe".to_string(),
        username: "johndoe".to_string(),
        verified: false,
        created_at: 1234567890,
    };
    println!("descriptor for User: {:#?}", u.entity_descriptor());
    println!("atlas:\n\n{}", hcl::to_string(&u.as_atlas_hcl()).unwrap());
}
