// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";

import {RouteKind} from "../../src/TraiectaTypes.sol";
import {RouteMeta} from "../../src/libraries/RouteMeta.sol";

/// @title Every rail against every question the router asks about it
/// @notice Four rails, three predicates, twelve answers.
/// @dev `RouteMeta` calls itself the mirror of the `impl RouteKind` block in
/// `hyperion_core::route`, and nothing in this repository compares the two. The answers below are
/// copied from the Rust, one function at a time, so a change on one chain that is not made on the
/// other lands here as a failing test rather than as an app that offers a rail the router then
/// refuses, or a quote that promises a muxed destination the rail cannot carry.
///
/// Two of the three predicates have a Rust counterpart. `carriesPayload` does not: the Soroban
/// router is never asked the question, because the EVM router refuses a muxed destination before
/// a message is ever built. Its answers come from `ROUTE_META` in
/// `packages/protocol/src/routes.ts`, which is the other half of this same table.
contract RouteMetaTest is Test {
    // -------------------------------------------------------------------------------------
    // waitsOnAttestation
    //
    // route.rs: `Cctp | AxelarIts | AxelarGmp => true, Allbridge => false`
    // -------------------------------------------------------------------------------------

    /// Circle's attestation is the wait everybody means when they say a bridge takes fifteen
    /// minutes, and it is the reason the router tells an app which rail it is on before it asks
    /// anyone to sign.
    function test_cctp_waits_on_an_attestation() public pure {
        assertTrue(RouteMeta.waitsOnAttestation(RouteKind.Cctp));
    }

    function test_axelar_its_waits_on_an_attestation() public pure {
        assertTrue(RouteMeta.waitsOnAttestation(RouteKind.AxelarIts));
    }

    function test_axelar_gmp_waits_on_an_attestation() public pure {
        assertTrue(RouteMeta.waitsOnAttestation(RouteKind.AxelarGmp));
    }

    /// Allbridge draws on a pool instead, which settles as fast as the destination chain does.
    /// Getting this one wrong is what makes an app show a fifteen minute estimate for a rail that
    /// takes fifteen seconds, and the reverse is worse.
    function test_allbridge_settles_from_a_pool_without_waiting() public pure {
        assertFalse(RouteMeta.waitsOnAttestation(RouteKind.Allbridge));
    }

    // -------------------------------------------------------------------------------------
    // isCanonical
    //
    // route.rs: `Cctp | AxelarIts => true, AxelarGmp | Allbridge => false`
    // -------------------------------------------------------------------------------------

    /// Burn and mint: what arrives is what left, minus a fee that was known before anybody signed.
    function test_cctp_moves_the_asset_itself() public pure {
        assertTrue(RouteMeta.isCanonical(RouteKind.Cctp));
    }

    function test_axelar_its_moves_the_asset_itself() public pure {
        assertTrue(RouteMeta.isCanonical(RouteKind.AxelarIts));
    }

    /// General message passing carries the canonical asset too, but it moves a payload that can
    /// hold a swap the sender did not ask for, so the app cannot promise no slippage.
    function test_axelar_gmp_is_not_canonical() public pure {
        assertFalse(RouteMeta.isCanonical(RouteKind.AxelarGmp));
    }

    /// A pooled rail can run thin, and the shortfall is the sender's.
    function test_allbridge_is_not_canonical() public pure {
        assertFalse(RouteMeta.isCanonical(RouteKind.Allbridge));
    }

    // -------------------------------------------------------------------------------------
    // carriesPayload
    //
    // routes.ts `ROUTE_META`: `Cctp, AxelarIts, AxelarGmp => true, Allbridge => false`.
    // This is the predicate the muxed destination check reads (router lines 253 and 395), so a
    // `true` here for Allbridge would promise a sixty four bit id it has nowhere to put.
    // -------------------------------------------------------------------------------------

    function test_cctp_can_carry_a_payload() public pure {
        assertTrue(RouteMeta.carriesPayload(RouteKind.Cctp));
    }

    function test_axelar_its_can_carry_a_payload() public pure {
        assertTrue(RouteMeta.carriesPayload(RouteKind.AxelarIts));
    }

    function test_axelar_gmp_can_carry_a_payload() public pure {
        assertTrue(RouteMeta.carriesPayload(RouteKind.AxelarGmp));
    }

    function test_allbridge_cannot_carry_a_payload() public pure {
        assertFalse(RouteMeta.carriesPayload(RouteKind.Allbridge));
    }

    // -------------------------------------------------------------------------------------
    // All twelve at once
    // -------------------------------------------------------------------------------------

    /// The table, in the order the two enums declare their members.
    ///
    /// The per rail functions above read better in a failure report, and this one is what makes a
    /// rail that was answered correctly in one predicate and wrongly in another impossible to
    /// miss: the arrays below are the Rust and the SDK side by side.
    function test_the_whole_table_matches_the_rust_and_the_sdk() public pure {
        RouteKind[4] memory routes = [
            RouteKind.Cctp,
            RouteKind.AxelarIts,
            RouteKind.AxelarGmp,
            RouteKind.Allbridge
        ];
        bool[4] memory waits = [true, true, true, false];
        bool[4] memory canonical = [true, true, false, false];
        bool[4] memory payload = [true, true, true, false];

        for (uint256 i = 0; i < routes.length; ++i) {
            assertEq(RouteMeta.waitsOnAttestation(routes[i]), waits[i], "waitsOnAttestation");
            assertEq(RouteMeta.isCanonical(routes[i]), canonical[i], "isCanonical");
            assertEq(RouteMeta.carriesPayload(routes[i]), payload[i], "carriesPayload");
        }
    }

    /// The two predicates that are not simply "true for everything canonical" are worth naming.
    ///
    /// A classifier that answered `true` to all three for all four rails would pass every test
    /// above that expects a yes. This is the one that would not.
    function test_the_rails_that_differ_are_the_two_that_should() public pure {
        assertTrue(
            RouteMeta.waitsOnAttestation(RouteKind.Allbridge) != RouteMeta.waitsOnAttestation(RouteKind.Cctp)
        );
        assertTrue(RouteMeta.isCanonical(RouteKind.Allbridge) != RouteMeta.isCanonical(RouteKind.Cctp));
        assertTrue(RouteMeta.isCanonical(RouteKind.AxelarGmp) != RouteMeta.isCanonical(RouteKind.AxelarIts));
        assertTrue(
            RouteMeta.carriesPayload(RouteKind.Allbridge) != RouteMeta.carriesPayload(RouteKind.Cctp)
        );
    }
}
