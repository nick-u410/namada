//! The oracle is controlled by sending commands over a channel.

use tokio::sync::mpsc;

use super::config::Config;

/// Used to send commands to an oracle.
pub type Sender = mpsc::Sender<Command>;
/// Used by an oracle to receive commands.
pub type Receiver = mpsc::Receiver<Command>;

/// Returns two sides of a [`mpsc`] channel that can be used for controlling an
/// oracle.
pub fn channel() -> (Sender, Receiver) {
    mpsc::channel(1)
}

/// Commands used to configure and control an `Oracle`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Command {
    /// Initializes the oracle with the given configuration and immediately
    /// starts it.
    Initialize { config: Config },
}
