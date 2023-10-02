//! Sent by Twitch for various reasons to notify the client about something,
//! usually in response to invalid actions.

use super::MessageParseError;
use crate::common::ChannelRef;
use crate::irc::{Command, IrcMessageRef, Tag};

/// Sent by TMI for various reasons to notify the client about something,
/// usually in response to invalid actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice<'src> {
  channel: Option<&'src ChannelRef>,
  text: &'src str,
  id: Option<&'src str>,
}

generate_getters! {
  <'src> for Notice<'src> as self {
    /// Target channel name.
    ///
    /// This may be empty before successful login.
    channel -> Option<&'src ChannelRef>,

    /// Notice message.
    text -> &'src str,

    /// Notice ID, see <https://dev.twitch.tv/docs/irc/msg-id/>.
    ///
    /// This will only be empty before successful login.
    id -> Option<&'src str>,
  }
}

impl<'src> Notice<'src> {
  fn parse(message: IrcMessageRef<'src>) -> Option<Self> {
    if message.command() != Command::Notice {
      return None;
    }

    Some(Notice {
      channel: message.channel(),
      text: message.text()?,
      id: message.tag(Tag::MsgId),
    })
  }
}

impl<'src> super::FromIrc<'src> for Notice<'src> {
  #[inline]
  fn from_irc(message: IrcMessageRef<'src>) -> Result<Self, MessageParseError> {
    Self::parse(message).ok_or(MessageParseError)
  }
}

impl<'src> From<Notice<'src>> for super::Message<'src> {
  fn from(msg: Notice<'src>) -> Self {
    super::Message::Notice(msg)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_notice_before_login() {
    assert_irc_snapshot!(Notice, ":tmi.twitch.tv NOTICE * :Improperly formatted auth");
  }

  #[test]
  fn parse_notice_basic() {
    assert_irc_snapshot!(Notice, "@msg-id=msg_banned :tmi.twitch.tv NOTICE #forsen :You are permanently banned from talking in forsen.");
  }
}
