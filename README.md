# graphix
An ORM for Rust heavily inspired by [ent](https://github.com/ent/ent).

## Example
```rust
use graphix::entity::Entity;
use graphix::Entity;

#[derive(Entity)]
struct User {
    #[graphix(colname = "user_id", unique, immutable)]
    id: i32,
    
    #[graphix(nullable)]
    name: String,
    age: i32,
}
```

## Todos
- [x] Basic entity definition and HCL generation
- [ ] Add support for `immutable`
- [ ] Full mappings between Rust and Postgres types
- [ ] Hook up to Atlas
- [ ] *so* many more things