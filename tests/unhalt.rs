use trillium::{Conn, Handler};
use trillium_nursery::unhalt;
use trillium_testing::{TestConn, assert_body, assert_status};

#[test]
fn later_handlers_run_and_can_override() {
    let handler = (unhalt("halting body"), |conn: Conn| async move {
        conn.with_status(418)
    });
    let mut conn = TestConn::build("get", "/", ()).on(&handler);
    assert_status!(conn, 418);
    assert_body!(conn, "halting body");
}

#[test]
fn without_unhalt_the_halt_stops_the_chain() {
    let handler = (
        "halting body",
        |conn: Conn| async move { conn.with_status(418) },
    );
    let conn = TestConn::build("get", "/", ()).on(&handler);
    assert_status!(conn, 200);
}

#[test]
fn a_non_halting_inner_handler_is_unaffected() {
    let handler = (
        unhalt(|conn: Conn| async move { conn.with_status(201) }),
        |conn: Conn| async move { conn.with_body("body") },
    );
    let mut conn = TestConn::build("get", "/", ()).on(&handler);
    assert_status!(conn, 201);
    assert_body!(conn, "body");
    assert!(!conn.is_halted());
}

#[test]
fn a_previously_halted_conn_stays_halted() {
    let handler = |conn: Conn| async move { unhalt(()).run(conn.halt()).await };
    let conn = TestConn::build("get", "/", ()).on(&handler);
    assert!(conn.is_halted());
}
