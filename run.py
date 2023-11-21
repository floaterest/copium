#!python3

import os, sys, subprocess, shutil

import pyperclip


def project() -> (str, str):
    """
    determine Rust or Haskell project
    returns source file and run command
    """
    files = set(os.listdir())
    if 'Cargo.toml' in files:
        return ('src/main.rs', shutil.which('cargo'))
    elif 'stack.yaml' in files:
        return ('src/Main.hs', shutil.which('stack'))
    else:
        raise Exception('No project found')

SRC, RUN = project()
INPUT = 'input.txt'


def run():
    with open(INPUT, 'r') as f:
        proc = subprocess.Popen([RUN, 'run'], stdin=f)


def help():
    pass

if __name__ == '__main__':
    help()
    command = sys.argv[1] if len(sys.argv) > 1 else ''
    while command != 'q':
        match command:
            case 'r':
                run()
            case 'q':
                pass
        command = input('>>> ')
