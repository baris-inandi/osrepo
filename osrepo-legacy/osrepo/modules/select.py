from modules.entry import Entry
from modules.fmt import Fmt
from modules.tools import abort, confirm


def select_os(search_result: list[str], elapsed: float):
    entries = []
    for index, entry in enumerate(search_result):
        current_entry = Entry(entry)
        entries.append(current_entry)
        print(Fmt.color("header", str(index + 1)), current_entry)
    print(f"\nsearch done in {elapsed}s.")
    user_selection = int(input("select an os (enter number): ")) - 1
    try:
        return entries[user_selection]
    except Exception:
        abort()


def select_os_version(entry: Entry):
    print("\nThe following versions are available for", entry.formatted_path())
    version_urls = []
    for index, version in enumerate(entry.versions):
        print(Fmt.color("header", str(index + 1)),
              entry.formatted_version(version))
        version_urls.append(version)
    user_selection = int(input("select version (enter number): ")) - 1
    try:
        return version_urls[user_selection]
    except Exception:
        abort()


def confirm_download(entry: Entry, version: str, browser: bool = False):
    download_url = entry.versions[version]["url"]
    print()
    print("Selected version:", Fmt.underline(version))

    if entry.is_community:
        Fmt.complain(
            "WARN",
            "This is a community OS. The download URL may not be reviewed.")
        Fmt.complain(
            "WARN",
            "Make sure you trust the author and check if the URL is safe.")
    if browser:
        Fmt.complain("WARN", "This entry does not have a direct download url.")
        Fmt.complain("WARN", "The url will be opened in your browser.")
        last_warning = "The following URL will be opened in your browser:"
    else:
        last_warning = "The OS will be downloaded from the following URL:"
    Fmt.complain("INFO", last_warning)
    Fmt.complain("INFO", Fmt.underline(download_url))
    if not confirm():
        abort("Aborted by user.")
    else:
        return {"url": download_url, "browser": browser}
