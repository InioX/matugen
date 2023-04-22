import subprocess
import pytest

from matugen.util import reload_apps


class TestReloadApps:
    # Tests that all commands in the commands list run successfully and in the correct order, and that the correct signals are sent to the correct processes.
    def test_happy_path_reload_apps(self, mocker):
        """
        Tests that all commands in the commands list run successfully and in the correct order,
        and that the correct signals are sent to the correct processes.
        """
        # Mock subprocess.run to return a successful result
        mocker.patch(
            "subprocess.run",
            return_value=subprocess.CompletedProcess(args=[], returncode=0),
        )

        # Call the function
        reload_apps()

        # Assert that subprocess.run was called with the correct arguments in the correct order
        subprocess.run.assert_has_calls(
            [
                mocker.call(
                    ["pkill", "-SIGUSR2", "waybar"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
                mocker.call(
                    [
                        "gsettings",
                        "set",
                        "org.gnome.desktop.interface",
                        "gtk-theme",
                        "adw-gtk3-dark",
                    ],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
                mocker.call(
                    ["pkill", "-SIGUSR1", "kitty"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
            ]
        )

    # Tests that the function handles commands that require elevated privileges correctly.
    def test_edge_case_elevated_privileges_reload_apps(self, mocker):
        """
        Tests that the function handles commands that require elevated privileges correctly.
        """
        # Mock subprocess.run to return a non-zero return code for the second command
        mocker.patch(
            "subprocess.run",
            side_effect=[
                subprocess.CompletedProcess(args=[], returncode=0),
                subprocess.CompletedProcess(args=[], returncode=1),
                subprocess.CompletedProcess(args=[], returncode=0),
            ],
        )

        # Call the function
        reload_apps()

        # Assert that subprocess.run was called with the correct arguments in the correct order
        subprocess.run.assert_has_calls(
            [
                mocker.call(
                    ["pkill", "-SIGUSR2", "waybar"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
                mocker.call(
                    [
                        "gsettings",
                        "set",
                        "org.gnome.desktop.interface",
                        "gtk-theme",
                        "adw-gtk3-dark",
                    ],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
                mocker.call(
                    ["pkill", "-SIGUSR1", "kitty"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
            ]
        )

    # Tests that the function does not have any unintended side effects.
    def test_general_behavior_reload_apps(self, mocker):
        """
        Tests that the function does not have any unintended side effects.
        """
        # Mock subprocess.run to return a successful result
        mocker.patch(
            "subprocess.run",
            return_value=subprocess.CompletedProcess(args=[], returncode=0),
        )

        # Call the function
        reload_apps()

        # Assert that subprocess.run was called with the correct arguments in the correct order
        subprocess.run.assert_has_calls(
            [
                mocker.call(
                    ["pkill", "-SIGUSR2", "waybar"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
                mocker.call(
                    [
                        "gsettings",
                        "set",
                        "org.gnome.desktop.interface",
                        "gtk-theme",
                        "adw-gtk3-dark",
                    ],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
                mocker.call(
                    ["pkill", "-SIGUSR1", "kitty"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ),
            ]
        )

        # Assert that there are no other calls to subprocess.run
        assert subprocess.run.call_count == 3
