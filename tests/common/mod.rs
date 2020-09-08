pub mod factories;
pub mod processor;

use diesel_migrations::{self};
diesel_migrations::embed_migrations!();

use store::DbConn;

pub fn setup() -> DbConn {
    let conn = conn();
    match std::env::var("TEST_ENV") {
        Ok(_) => (),
        //should run only once
        Err(_) => {
            std::env::set_var("TEST_ENV", "true");
            std::env::set_var(
                "DATABASE_URL",
                std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL env var"),
            );
            recreate_db(&conn);
            setup_suite();
        }
    };

    clean_db(&conn);
    return conn;
}

pub fn setup_suite() {
    //enable to debug tests
    //webphone::common::pretty_env_logger::init_timed();
}

//TODO: improve here
fn recreate_db(conn: &DbConn) {
    use diesel::RunQueryDsl;

    diesel::sql_query("DROP DATABASE IF EXISTS \"webphone-test\"")
        .execute(conn)
        .expect("Dropping webphone-test");
    diesel::sql_query("CREATE DATABASE \"webphone-test\"")
        .execute(conn)
        .expect("Creating webphone-test");
}

pub fn conn() -> DbConn {
    store::db_conn().expect("db conn")
}

pub fn clean_db(conn: &DbConn) {
    use diesel::RunQueryDsl;
    use store::schema::auth_requests;
    use store::schema::dialogs;
    use store::schema::registrations;
    use store::schema::requests;
    use store::schema::responses;
    use store::schema::transactions;

    embedded_migrations::run_with_output(conn, &mut std::io::stdout()).expect("running migrations");
    diesel::delete(auth_requests::table)
        .execute(conn)
        .expect("deleting auth_requests");
    diesel::delete(dialogs::table)
        .execute(conn)
        .expect("deleting dialogs");
    diesel::delete(registrations::table)
        .execute(conn)
        .expect("deleting registrations");
    diesel::delete(requests::table)
        .execute(conn)
        .expect("deleting requests");
    diesel::delete(responses::table)
        .execute(conn)
        .expect("deleting responses");
    diesel::delete(responses::table)
        .execute(conn)
        .expect("deleting responses");
}

/*
pub async fn advance_for(seconds: i64) {
    tokio::time::pause();
    tokio::time::advance(
        chrono::Duration::seconds(60)
            .to_std()
            .expect("chrono to std"),
    )
    .await;
    tokio::time::resume();
}*/
