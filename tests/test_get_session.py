import subprocess
import pytest

from matugen.util import get_session

class TestGetSession:
    # Tests that the function returns a valid session string when the user is logged in and has a session.
    def test_get_session_happy_path_logged_in_with_session(self, mocker):
        # Mock subprocess.run to return a valid session string
        mocker.patch(
            "subprocess.run",
            return_value=subprocess.CompletedProcess(
                args="", returncode=0, stdout=b"wayland\n"
            ),
        )
        assert get_session() == "wayland"

    # Tests that the function returns an empty string when the user is not logged in.
    def test_get_session_edge_case_not_logged_in(self, mocker):
        # Mock subprocess.run to return an empty string
        mocker.patch(
            "subprocess.run",
            return_value=subprocess.CompletedProcess(
                args="", returncode=1, stdout=b"", stderr=b""
            ),
        )
        assert get_session() == ""

    # Tests that the function returns an empty string when the user is logged in but session information is not available.
    def test_get_session_edge_case_logged_in_without_session(self, mocker):
        # Mock subprocess.run to return a valid session string but without session information
        mocker.patch(
            "subprocess.run",
            return_value=subprocess.CompletedProcess(
                args="", returncode=0, stdout=b"\n"
            ),
        )
        assert get_session() == ""
