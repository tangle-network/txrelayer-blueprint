// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.20;

import {Test} from "forge-std/Test.sol";

import "./SetMessage.sol";

contract SetMessageTest is Test {
    SetMessage sm;

    function setUp() public {
        sm = new SetMessage();
        sm.set("Hello, World!");
    }

    function testGet() public view {
        assertEq(sm.get(), "Hello, World!");
    }
}
