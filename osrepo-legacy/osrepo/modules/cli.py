import click
from modules import run
from modules.download import download


@click.command(help="Lists all available OS entries")
def list():
    run.list_all()


@click.command(help="Searches the repository for a given keyword")
@click.argument('query')
def search(query: str):
    download(run.search_os(query))


@click.command(help="Installs OS from given identifier")
@click.argument('_id')
def install(_id: str):
    download(run.download_with_id(_id))


@click.group(help="CLI tool to manage full development cycle of projects")
def cli():
    pass


commands = [list, search, install, update]

for command in commands:
    cli.add_command(command)
