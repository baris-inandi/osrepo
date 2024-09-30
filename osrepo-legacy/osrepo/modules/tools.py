import inquirer


def abort(msg: str = "Nothing to do."):
    print(msg)
    quit()


def confirm(msg: str = "Do you want to proceed?", default: bool = True):
    return inquirer.prompt(
        [inquirer.Confirm("is_confirmed", message=msg,
                          default=default)])["is_confirmed"]
