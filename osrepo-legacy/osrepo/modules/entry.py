from modules.repo import repo
from modules.fmt import Fmt


class Entry:

    def __init__(self, name: str):
        self.name = str(name)
        self.meta = repo[name]
        self.versions = repo[name]['versions']
        self.is_community = repo[name]["repo"] == "community"

    # communitY/someOS
    def formatted_path(self):
        repo_color = "cyan" if self.meta["repo"] == "core" else "red"
        return f"{Fmt.color(repo_color, self.meta['repo'])}/{self.name}"

    # [proprietary][published by baris-inandi] some os that can do some things
    def formatted_description(self):
        if self.is_community:
            added_by = f"{self.meta['added_by']}"
            desc_trail = Fmt.color("green", f"[published by {added_by}] ")
        else:
            desc_trail = ""
        try:
            # the "is True" is required because ["proprietary"] can be None
            if self.meta["proprietary"] is True:
                proprietary_warning = Fmt.color("yellow", "[proprietary] ")
        except Exception:
            proprietary_warning = ""
        desc_trail = proprietary_warning + desc_trail
        return f"  {desc_trail}{self.meta['description'][:150]}"

    # [19 versions available]
    def formatted_version_count(self):
        versions_count = len(self.meta["versions"])
        return (Fmt.color("blue", f" [{versions_count} versions available]")
                if versions_count > 1 else Fmt.color(
                    "blue", f" {list(self.meta['versions'].keys())[0]}"))

    # community/someOS@v1.0.0 [added by baris-inandi]
    def formatted_version(self, version: str):
        version_contributor = ""
        if self.is_community:
            version_contributor = self.meta["versions"][version]["contributor"]
        return self.formatted_path(
        ) + Fmt.underline("@" + version) + Fmt.color(
            "green",
            f" [added by {version_contributor}]" if self.is_community else "")

    def __repr__(self):
        return self.formatted_path() + self.formatted_version_count(
        ) + "\n" + self.formatted_description()
