use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
};
use trillium::{Conn, Handler, Info, Upgrade};

/// A [`Handler`] that runs its inner handler only for conns matched by a predicate. See [`when`].
pub struct When<F, H> {
    predicate: F,
    handler: H,
}

/// Runs `handler` only for conns on which `predicate` returns true.
///
/// Only [`Handler::run`] is conditional. The other lifecycle hooks are delegated unconditionally,
/// matching the convention that `before_send` and the upgrade hooks guard themselves on state
/// that only their own `run` places: when `run` was skipped, those hooks find no state and do
/// nothing.
///
/// ```
/// use trillium_nursery::when;
///
/// let handler = when(
///     |conn: &trillium::Conn| conn.path() != "/webhook",
///     "this is not the webhook",
/// );
/// ```
pub fn when<F, H>(predicate: F, handler: H) -> When<F, H>
where
    F: Fn(&Conn) -> bool + Send + Sync + 'static,
    H: Handler,
{
    When { predicate, handler }
}

impl<F, H> Handler for When<F, H>
where
    F: Fn(&Conn) -> bool + Send + Sync + 'static,
    H: Handler,
{
    async fn run(&self, conn: Conn) -> Conn {
        if (self.predicate)(&conn) {
            self.handler.run(conn).await
        } else {
            conn
        }
    }

    async fn init(&mut self, info: &mut Info) {
        self.handler.init(info).await;
    }

    async fn before_send(&self, conn: Conn) -> Conn {
        self.handler.before_send(conn).await
    }

    fn has_upgrade(&self, upgrade: &Upgrade) -> bool {
        self.handler.has_upgrade(upgrade)
    }

    async fn upgrade(&self, upgrade: Upgrade) {
        self.handler.upgrade(upgrade).await;
    }

    fn name(&self) -> Cow<'static, str> {
        format!("When({})", self.handler.name()).into()
    }
}

impl<F, H: Handler> Debug for When<F, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("When")
            .field("predicate", &"Fn(&Conn) -> bool")
            .field("handler", &self.handler.name())
            .finish()
    }
}
