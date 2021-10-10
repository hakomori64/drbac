fn main() {
    use common::db::utils::establish_connection;
    use common::db::models::actor;
    let conn = establish_connection().unwrap();
    let actors = actor::get_actors(&conn).unwrap();
    for actor in actors {
        println!("{:?}", actor);
    }
}