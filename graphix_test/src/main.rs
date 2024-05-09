use graphix::entity::Entity;
use graphix::Entity;

#[derive(Entity)]
#[graphix(schema_name = "foobar")]
struct User {
    #[graphix(immutable, nullable, colname = "urmum")]
    foo: String,
}

fn main() {
    let u = User {
        foo: String::from("bar"),
    };
    println!("descriptor for User: {:#?}", u.entity_descriptor());
    println!("atlas: {:#?}", u.as_atlas_hcl());
}
