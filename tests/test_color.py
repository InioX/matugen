from matugen.util import Color

import pytest


class TestColor:
    # Tests that rgb_to_hex() returns the correct hex string for a valid RGB value.
    def test_rgb_to_hex_happy_path(self):
        assert Color.rgb_to_hex((255, 255, 255)) == "ffffff"
        assert Color.rgb_to_hex((0, 0, 0)) == "000000"
        assert Color.rgb_to_hex((128, 128, 128)) == "808080"

    # Tests that hex_to_rgb() returns the correct RGB tuple for a valid hex string with minimum value.
    def test_hex_to_rgb_edge_case(self):
        assert Color.hex_to_rgb("000000") == (0, 0, 0)

    # Tests that dec_to_rgb() returns the correct RGB tuple for a valid decimal value.
    def test_dec_to_rgb_general_behavior(self):
        assert Color.dec_to_rgb(16777215) == (255, 255, 255)
        assert Color.dec_to_rgb(0) == (0, 0, 0)
        assert Color.dec_to_rgb(8421504) == (128, 128, 128)
