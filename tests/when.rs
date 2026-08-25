use trillium::{Conn, Handler};
use trillium_nursery::when;
use trillium_testing::{TestConn, assert_body, assert_not_handled, assert_status};

#[test]
fn matched_conns_run_the_handler() {
    let handler = when(|conn: &Conn| conn.path() == "/matched", "ran");
    let mut conn = TestConn::build("get", "/matched", ()).on(&handler);
    assert_status!(conn, 200);
    assert_body!(conn, "ran");
}

#[test]
fn unmatched_conns_skip_the_handler() {
    let handler = when(|conn: &Conn| conn.path() == "/matched", "ran");
    let conn = TestConn::build("get", "/other", ()).on(&handler);
    assert_not_handled!(conn);
}

#[test]
fn nested_whens_compose() {
    let handler = when(
        |conn: &Conn| conn.path().starts_with("/a"),
        when(|conn: &Conn| conn.path().ends_with("/b"), "both matched"),
    );

    let mut conn = TestConn::build("get", "/a/b", ()).on(&handler);
    assert_body!(conn, "both matched");

    let conn = TestConn::build("get", "/a/c", ()).on(&handler);
    assert_not_handled!(conn);

    let conn = TestConn::build("get", "/x/b", ()).on(&handler);
    assert_not_handled!(conn);
}

struct SelfGuarding;
struct Ran;

impl Handler for SelfGuarding {
    async fn run(&self, conn: Conn) -> Conn {
        conn.with_state(Ran)
    }

    async fn before_send(&self, conn: Conn) -> Conn {
        if conn.state::<Ran>().is_some() {
            conn.with_response_header("x-ran", "true")
        } else {
            conn
        }
    }
}

#[test]
fn before_send_is_delegated_unconditionally_and_self_guards() {
    let handler = when(|conn: &Conn| conn.path() == "/matched", SelfGuarding);

    let conn = TestConn::build("get", "/matched", ()).on(&handler);
    assert_eq!(conn.response_headers().get_str("x-ran"), Some("true"));

    let conn = TestConn::build("get", "/other", ()).on(&handler);
    assert_eq!(conn.response_headers().get_str("x-ran"), None);
}
