from modules import select
from modules.search import search
from modules.tools import abort
from modules.repo import repo
from modules.entry import Entry
from modules.fmt import Fmt
from modules.tools import confirm
from modules.select import select_os_version, confirm_download


def search_os(keyword: str):
    # osr list Arch
    # returns the download URL of selected os/version.
    search_result, elapsed = search(keyword)
    if len(search_result) > 0:
        selected_os = select.select_os(search_result, elapsed)
        selected_version = select.select_os_version(selected_os)
        download_url = select.confirm_download(selected_os, selected_version)
    else:
        abort("No results found.")
    return download_url


def list_all():
    # osr list *
    if confirm(msg="This will print out *ALL* OS entries. Are you sure?"):
        for index, name in enumerate(repo.keys()):
            print(Fmt.color("header", str(index + 1)), Entry(name), "\n")
    else:
        abort("Aborted by user.")


def download_with_id(os_id: str):
    # osr install Windows@10
    # osr install Windows
    try:
        os_id_list = os_id.split("@", 1)
        os_name = os_id_list[0]
        current_entry = Entry(os_name)

        version_specified = len(os_id_list) >= 2
        if version_specified:
            print(current_entry.formatted_version(os_id_list[1]))
            version = os_id_list[1]
        else:
            version = select_os_version(current_entry)
        return confirm_download(
            current_entry,
            version,
        )

    except Exception:
        abort()
