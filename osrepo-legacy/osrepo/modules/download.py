from os import system

def download(d: dict):
    print()
    print("Starting download...")
    system(f"wget {d['url']}")
