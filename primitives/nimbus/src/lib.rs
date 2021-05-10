//TODO license

//! Nimbus Consensus Primitives
//!
//! TODO rename the crate. It was originally just the runtime api

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use parity_scale_codec::Codec;
use sp_application_crypto::KeyTypeId;

//TODO Maybe move our key type into sp_core if this gets well adopted (to avoid collision)
pub const NIMBUS_KEY_ID: KeyTypeId = KeyTypeId(*b"nmbs");

/// A mechanism for determining the current slot.
/// For now we use u32 as the slot type everywhere. Let's see how long we can get away with that.
pub trait SlotBeacon {
    fn slot() -> u32;
}

/// Trait to determine whether this author is eligible to author in this slot.
/// This is the primary trait your nimbus filter needs to implement.
///
/// This is the proposition-logic variant.
/// That is to say the caller specifies an author an author and the implementation
/// replies whether that author is eligible. This is useful in many cases and is
/// particularly useful when the active set is unbounded.
/// There may be another variant where the caller only supplies a slot and the
/// implementation replies with a complete set of eligible authors.
pub trait CanAuthor<AuthorId> {
	fn can_author(author: &AuthorId, slot: &u32) -> bool;
}
/// Default implementation where anyone can author.
// TODO Promote this is "implementing relay chain consensus in the nimbus framework."
impl<T> CanAuthor<T> for () {
	fn can_author(_: &T, _: &u32) -> bool {
		true
	}
}

//TODO revisit all of this app crypto stuff and make it nice and clean
mod app {
	use sp_application_crypto::{
		app_crypto,
		sr25519,
	};
	app_crypto!(sr25519, crate::NIMBUS_KEY_ID);
}

sp_application_crypto::with_pair! {
	/// A nimbus author keypair.
	pub type NimbusPair = app::Pair;
}

/// A nimbus author identifier.
pub type NimbusId = app::Public;

/// A nimbus author signature.
pub type NimbusSignature = app::Signature;


sp_api::decl_runtime_apis! {
    /// the runtime api used to predict whether an author will be eligible in the given slot
    pub trait AuthorFilterAPI<AuthorId: Codec> {
        fn can_author(author: AuthorId, relay_parent: u32) -> bool;
    }
}

/// Idea shelved.
/// it is possible to make the runtime API give more details about why an author is ineligible.
/// Specifically it could distinguish between failing the prechecks and the full checks. But in the
/// who cares (except maybe for debugging). The author can't author, and there's no reason to call
/// both checks.
/// One possible reason is if the full check is considerably more expensive. Anyway, it's shelved
/// for now.
#[allow(dead_code)]
enum AuthorCheckResult {
    /// Author does not even pass the preliminaty checks.
    FailsPreliminaryChecks,
    /// Author passes preliminary checks, but not full checks.
    FailsFullChekcs,
    /// Author is eligible at this slot.
    Eligible,
}
