use anyhow::Result;
use common::db::utils::establish_connection;
use common::db::models::actor::generate_actor_id;
use common::db::models::entity::create_entity;
use common::db::models::role::create_role;
use common::db::models::user::create_user;
use common::pki::generate_key_pair;
use names::Generator;

fn main() -> Result<()> {

    let mut generator = Generator::default();
    let conn = establish_connection()?;
    let (secret_key, public_key) = generate_key_pair()?;
    let entity_id = generate_actor_id()?;
    create_entity(
        &conn,
        entity_id.clone(),
        "entity_".to_string() + generator.next().unwrap().as_str(),
        Some(secret_key.clone()),
        Some(public_key.clone())
    )?;

    let role_name = "role_".to_string() + generator.next().unwrap().as_str();
    create_role(
        &conn,
        generate_actor_id()?,
        entity_id.clone(),
        role_name.clone(),
        false,
        Some(secret_key.clone()),
        Some(public_key.clone())
    )?;

    create_role(
        &conn,
        generate_actor_id()?,
        entity_id.clone(),
        role_name.clone(),
        true,
        Some(secret_key.clone()),
        Some(public_key.clone())
    )?;

    create_user(
        &conn,
        generate_actor_id()?,
        entity_id.clone(),
        "user_".to_string() + generator.next().unwrap().as_str(),
        Some(secret_key.clone()),
        Some(public_key.clone())
    )?;

    Ok(())
}