import sublime
import sublime_plugin

class PackageCompletionsListener(sublime_plugin.EventListener):
    def on_query_completions(self, view, prefix, locations):
        # Check if we're in a Radium file
        if not view.match_selector(locations[0], "source.radium"):
            return None

        # Get the current line's content up to the cursor position
        cursor_pos = locations[0]  # Assume single cursor for simplicity
        line_region = view.line(cursor_pos)
        line_text_up_to_cursor = view.substr(sublime.Region(line_region.a, cursor_pos))

        # Check if the cursor is immediately after "package "
        if not line_text_up_to_cursor.endswith("package "):
            return None

        # Completions to show after 'package'
        package_completions = [
            ("radium\tPackage", "radium"),
            ("async\tPackage", "async"),
            ("web\tPackage", "web"),
            ("system\tPackage", "system")
        ]

        # Trigger the completions
        return (package_completions, sublime.INHIBIT_WORD_COMPLETIONS | sublime.INHIBIT_EXPLICIT_COMPLETIONS)