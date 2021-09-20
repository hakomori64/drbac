pub mod stream;
pub mod encoding;
pub mod io;
pub mod crypto;
pub mod connection;
pub mod messages;
pub mod actors;
pub mod pki;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn check_is_valid_actor_format_success() {
        use crate::actors::utils;
        assert_eq!(
            utils::is_valid_actor_format(
                &String::from("EntityA.UserA")),
            true
        );
        assert_eq!(
            utils::is_valid_actor_format(
                &String::from("EntityA")),
            true
        );
    }

    #[test]
    fn check_is_valid_actor_format_fail() {
        use crate::actors::utils;
        assert_ne!(
            utils::is_valid_actor_format(
                &String::from("EntityA UserA")),
            true
        );
        assert_ne!(
            utils::is_valid_actor_format(
                &String::from(r#"EntityA\UserA"#)),
            true
        );
        assert_ne!(
            utils::is_valid_actor_format(
                &String::from("EntityA UserA And User B")),
            true
        );
        assert_ne!(
            utils::is_valid_actor_format(
                &String::from("EntityA/UserA")),
            true
        );
    }

    #[test]
    fn check_is_valid_format() {
        use crate::actors::Actor;
        use crate::actors::utils;
        assert_eq!(
            utils::is_valid_format(&Actor::Entity, &String::from("EntityA")),
            true
        );
        assert_eq!(
            utils::is_valid_format(&Actor::Role, &String::from("EntityA.RoleA")),
            true
        );
        assert_eq!(
            utils::is_valid_format(&Actor::User, &String::from("EntityA.UserA")),
            true
        );
    }

    #[test]
    fn check_is_valid_format_fail() {
        use crate::actors::Actor;
        use crate::actors::utils;
        assert_ne!(
            utils::is_valid_format(&Actor::Entity, &String::from("EntityA.Role!")),
            true
        );
        assert_ne!(
            utils::is_valid_format(&Actor::Role, &String::from("EntityA.Role Role!")),
            true
        );
        assert_ne!(
            utils::is_valid_format(&Actor::User, &String::from("EntityA")),
            true
        );
    }

    #[test]
    fn craft_base_dir_success() {
        use std::path::PathBuf;
        use crate::actors::Actor;
        use crate::actors::utils;
        assert_eq!(
            utils::craft_base_dir(&Actor::Entity, &String::from("hello")).unwrap(),
            ["actors", "hello"].iter().collect::<PathBuf>()
        );
        assert_eq!(
            utils::craft_base_dir(&Actor::Role, &String::from("entity.role")).unwrap(),
            ["actors", "entity", "roles", "role"].iter().collect::<PathBuf>()
        );
        assert_eq!(
            utils::craft_base_dir(&Actor::User, &String::from("entity.user")).unwrap(),
            ["actors", "entity", "users", "user"].iter().collect::<PathBuf>()
        );
    }

    #[test]
    fn get_secret_key_path_success() {
        use crate::actors::Actor;
        use crate::actors::utils;
        use std::path::PathBuf;

        assert_eq!(
            utils::get_secret_key_path(&Actor::Entity, &String::from("entityA")).unwrap(),
            ["actors", "entityA", "secret.pem"].iter().collect::<PathBuf>()
        );
        assert_eq!(
            utils::get_secret_key_path(&Actor::Role, &String::from("entityA.role")).unwrap(),
            ["actors", "entityA", "roles", "role", "secret.pem"].iter().collect::<PathBuf>()
        );
        assert_eq!(
            utils::get_secret_key_path(&Actor::User, &String::from("entityA.user")).unwrap(),
            ["actors", "entityA", "users", "user", "secret.pem"].iter().collect::<PathBuf>()
        );
    }
}
