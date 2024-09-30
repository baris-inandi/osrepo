class Fmt:
    colors = {
        "HEADER": '\033[95m',
        "BLUE": '\033[94m',
        "CYAN": '\033[96m',
        "GREEN": '\033[92m',
        "YELLOW": '\033[93m',
        "RED": '\033[91m',
        "DEFAULT": '\033[0m',
        "BOLD": '\033[1m',
        "UNDERLINE": '\033[4m'
    }

    complaints = {
        "INFO": colors["CYAN"],
        "WARN": colors["YELLOW"],
        "ERROR": colors["RED"]
    }

    @classmethod
    def complain(cls, level: str, msg: str):
        level = level.upper()
        if level not in cls.complaints:
            return ""
        return print(
            cls.bold(cls.complaints[level] + level + ": " +
                     cls.colors["DEFAULT"]) + msg)

    @classmethod
    def underline(cls, msg: str):
        return cls.colors["UNDERLINE"] + msg + cls.colors["DEFAULT"]

    @classmethod
    def bold(cls, msg: str):
        return cls.colors["BOLD"] + msg + cls.colors["DEFAULT"]

    @classmethod
    def color(cls, color: str, msg: str):
        return cls.colors[color.upper()] + msg + cls.colors["DEFAULT"]
