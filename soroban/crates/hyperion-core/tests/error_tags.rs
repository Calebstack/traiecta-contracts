//! The numbers `HyperionError` hands to the world, one variant at a time.
//!
//! These tags are not private detail. `ContractError` turns them into sentences in the SDK, the
//! indexer stores them against a transfer record, and the app renders them, so `31` has to mean
//! the same thing everywhere it has ever been written down. `error.rs` says the numbers are
//! stable and nothing enforced it: the parity suite in `packages/protocol` compares the variant
//! names and the count against the TypeScript table, so quietly renumbering `TokenNotRegistered`
//! moved it in both places at once and stayed green across the workspace.
//!
//! That is the failure this file exists to turn red. It is deliberately boring: one line per
//! variant, hand written, and no code that would follow the enum if the enum moved.

use hyperion_core::HyperionError;

/// Every variant with the tag it was published with, in declaration order.
///
/// Written out rather than derived. A table that read the numbers off the enum would move with
/// it, which is the opposite of what a stability check has to do.
#[rustfmt::skip]
const TAGS: &[(&str, HyperionError, u32)] = &[
    ("AlreadyInitialized", HyperionError::AlreadyInitialized, 1),
    ("NotInitialized", HyperionError::NotInitialized, 2),
    ("Unauthorized", HyperionError::Unauthorized, 3),
    ("NotRailReceiver", HyperionError::NotRailReceiver, 4),
    ("Paused", HyperionError::Paused, 5),
    ("RouteDisabled", HyperionError::RouteDisabled, 6),
    ("AdapterNotSet", HyperionError::AdapterNotSet, 7),
    ("FlowLimitExceeded", HyperionError::FlowLimitExceeded, 8),
    ("InvalidAmount", HyperionError::InvalidAmount, 9),
    ("AmountNotRepresentable", HyperionError::AmountNotRepresentable, 10),
    ("DecimalOverflow", HyperionError::DecimalOverflow, 11),
    ("InvalidDecimals", HyperionError::InvalidDecimals, 12),
    ("SlippageExceeded", HyperionError::SlippageExceeded, 13),
    ("FeeTooHigh", HyperionError::FeeTooHigh, 14),
    ("InvalidDestination", HyperionError::InvalidDestination, 15),
    ("ZeroAddressKey", HyperionError::ZeroAddressKey, 16),
    ("MuxedNotSupported", HyperionError::MuxedNotSupported, 17),
    ("NotEvmAddress", HyperionError::NotEvmAddress, 18),
    ("UnknownChain", HyperionError::UnknownChain, 19),
    ("ReplayedMessage", HyperionError::ReplayedMessage, 20),
    ("UnknownNonce", HyperionError::UnknownNonce, 21),
    ("TimelockNotQueued", HyperionError::TimelockNotQueued, 22),
    ("TimelockNotReady", HyperionError::TimelockNotReady, 23),
    ("TimelockExpired", HyperionError::TimelockExpired, 24),
    ("TimelockDelayOutOfRange", HyperionError::TimelockDelayOutOfRange, 25),
    ("ClaimNotFound", HyperionError::ClaimNotFound, 26),
    ("ClaimAlreadySettled", HyperionError::ClaimAlreadySettled, 27),
    ("RecipientNotReady", HyperionError::RecipientNotReady, 28),
    ("InvalidLimit", HyperionError::InvalidLimit, 29),
    ("InvalidWindow", HyperionError::InvalidWindow, 30),
    ("TokenNotRegistered", HyperionError::TokenNotRegistered, 31),
    ("MalformedMessage", HyperionError::MalformedMessage, 32),
    ("UnsupportedHookVersion", HyperionError::UnsupportedHookVersion, 33),
    ("RailNotConfigured", HyperionError::RailNotConfigured, 34),
    ("WrongDomain", HyperionError::WrongDomain, 35),
    ("NotMintRecipient", HyperionError::NotMintRecipient, 36),
    ("UnexpectedRailContract", HyperionError::UnexpectedRailContract, 37),
    ("NothingMinted", HyperionError::NothingMinted, 38),
    ("TokenNotMapped", HyperionError::TokenNotMapped, 39),
    ("AlreadyConfigured", HyperionError::AlreadyConfigured, 40),
    ("InsufficientLiquidity", HyperionError::InsufficientLiquidity, 41),
    ("UnsupportedMessageVersion", HyperionError::UnsupportedMessageVersion, 42),
    ("NotTheRail", HyperionError::NotTheRail, 43),
    ("UnsupportedRoute", HyperionError::UnsupportedRoute, 44),
    ("GasFloatTooLow", HyperionError::GasFloatTooLow, 45),
    ("ProtectedAsset", HyperionError::ProtectedAsset, 46),
];

/// The last tag in the enum today. No variant may sit above this without the table growing with
/// it, which is what the length check below is for.
const HIGHEST_TAG: u32 = 46;

/// Every variant still carries the number it was published with.
#[test]
fn every_variant_keeps_the_number_it_was_published_with() {
    for (name, variant, tag) in TAGS {
        assert_eq!(
            *variant as u32, *tag,
            "HyperionError::{name} has moved to {} and someone may already have stored {tag} \
             against a transfer",
            *variant as u32,
        );
    }
}

/// The table is in declaration order, so an insertion in the middle cannot hide.
///
/// This is the one that catches a new variant slotted between two existing ones: every tag from
/// the insertion down then differs from the position it sits at, even if the enum itself is
/// renumbered to stay contiguous.
#[test]
fn the_table_is_in_declaration_order() {
    for (position, (name, _, tag)) in TAGS.iter().enumerate() {
        let expected = position as u32 + 1;
        assert_eq!(
            *tag, expected,
            "the entry at position {position} ({name}) carries tag {tag}",
        );
    }
}

/// The tags run from one to the last one with no gap and no repeat.
///
/// A gap is a variant somebody deleted and later reused, which relabels history. A repeat is the
/// same tag doing two jobs, which the compiler happens to refuse today and should keep being
/// refused if these numbers ever move out of a `repr(u32)` enum.
#[test]
fn the_numbers_run_from_one_with_no_gap_and_no_repeat() {
    let mut numbers: Vec<u32> = TAGS.iter().map(|(_, _, tag)| *tag).collect();
    let written = numbers.len();
    numbers.sort_unstable();
    numbers.dedup();
    assert_eq!(numbers.len(), written, "two variants claim the same tag");
    for (position, number) in numbers.iter().enumerate() {
        let expected = position as u32 + 1;
        assert_eq!(*number, expected, "expected {expected} at position {position}");
    }
}

/// The table names every variant the enum declares, so a silent addition is caught here too.
#[test]
fn the_table_names_each_variant_once_and_reaches_the_last_tag() {
    assert_eq!(
        TAGS.len() as u32,
        HIGHEST_TAG,
        "the table and HIGHEST_TAG disagree about how many variants there are",
    );
    let mut names: Vec<&str> = TAGS.iter().map(|(name, _, _)| *name).collect();
    let written = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), written, "a variant is listed twice");
}
