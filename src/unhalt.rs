use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
};
use trillium::{Conn, Handler, Info, Upgrade};

/// A [`Handler`] that prevents its inner handler from halting the conn. See [`unhalt`].
pub struct Unhalt<H>(H);

/// Runs `handler` but prevents it from halting the conn, so later handlers in the sequence still
/// run. Use this to compose a handler someone else wrote that unconditionally halts when you want
/// subsequent handlers to have a chance to modify the response.
///
/// This does not undo anything else the inner handler did: any status, body, headers, or state it
/// set flow onward, and a later handler overrides them only if it sets them itself. A conn that
/// was already halted before this handler stays halted.
///
/// ```
/// use trillium_nursery::unhalt;
///
/// // "not found" halts with a 200; the second handler now runs anyway and overrides the status
/// let handler = (
///     unhalt("not found"),
///     |conn: trillium::Conn| async move { conn.with_status(404) },
/// );
/// ```
pub fn unhalt<H: Handler>(handler: H) -> Unhalt<H> {
    Unhalt(handler)
}

impl<H: Handler> Handler for Unhalt<H> {
    async fn run(&self, conn: Conn) -> Conn {
        let halted_on_entry = conn.is_halted();
        let mut conn = self.0.run(conn).await;
        conn.set_halted(halted_on_entry);
        conn
    }

    async fn init(&mut self, info: &mut Info) {
        self.0.init(info).await;
    }

    async fn before_send(&self, conn: Conn) -> Conn {
        self.0.before_send(conn).await
    }

    fn has_upgrade(&self, upgrade: &Upgrade) -> bool {
        self.0.has_upgrade(upgrade)
    }

    async fn upgrade(&self, upgrade: Upgrade) {
        self.0.upgrade(upgrade).await;
    }

    fn name(&self) -> Cow<'static, str> {
        format!("Unhalt({})", self.0.name()).into()
    }
}

impl<H: Handler> Debug for Unhalt<H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Unhalt").field(&self.0.name()).finish()
    }
}
