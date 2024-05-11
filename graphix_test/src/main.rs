use graphix::entity::Entity;
use graphix::Entity;

#[derive(Entity)]
#[graphix(schema_name = "foobar", table_name = "urmum")]
struct User {
    #[graphix(immutable, unique, nullable, colname = "urmum")]
    foo: bool,
}

fn main() {
    let u = User {
        foo: true
    };
    println!("descriptor for User: {:#?}", u.entity_descriptor());
    println!("atlas:\n\n{}", hcl::to_string(&u.as_atlas_hcl()).unwrap());
}
