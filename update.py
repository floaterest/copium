#!/usr/bin/python3

import os, argparse
from os import path
from codecs import open
from dataclasses import dataclass
from enum import Enum

class Platform(Enum):
    LEETCODE = 'leetcode.com'

@dataclass
class Problem:
    domain: str
    contest: str
    task: str
    def __init__(self, domain: Platform, contest: str, task: str):
        self.domain = domain.value
        self.contest = contest
        self.task = task
    def __str__(self):
        match Platform(self.domain):
            case Platform.LEETCODE:
                return f'https://leetcode.com/problems/{self.task}/'

def convert(url: str) -> Problem:
    parts = url.removeprefix('https://').removesuffix('/').split('/')
    domain = Platform(parts[0])
    match domain:
        case Platform.LEETCODE:
            # https://leetcode.com/problems/<task>/*
            return Problem(domain, '', parts[2])


def write(file: str, dest: str, url: str):
    match os.path.splitext(file)[1]:
        case '.rs':
            prefix = '//'
        case '.hs':
            prefix = '--'
    with open(file, 'r', 'utf8') as fi, open(dest, 'w', 'utf8') as fo:
        fo.write(f'{prefix} {url}\n')
        fo.write(fi.read())


def main(args):
    url = args.url.lower()
    p = convert(url)
    url = str(p)

    ext = path.splitext(args.src)[1]
    dest = path.dirname(path.realpath(__file__))
    dest = path.join(dest, p.domain, p.contest, p.task) + ext
    
    if path.exists(dest):
        act, copy = 'update', '\033[93mReplace\033[00m'
    else:
        act, copy = 'add', '\033[92mCopy\033[00m'

    # copy file
    print(copy, args.src, '-->', dest)
    if args.dry:
        system = print
    else:
        system = os.system
        os.makedirs(path.dirname(dest), exist_ok=True)
        write(args.src, dest, url)
    # git
    system(f'git add {dest}')
    comment = f' ({args.comment})' if args.comment else ''
    task = path.basename(dest)
    contest = p.contest or p.domain
    cmd = f'git commit -m "{act}({contest}) {task}{comment}" -m "{url}"'
    system(cmd)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('src', type=str, help='path to submission code')
    parser.add_argument('url', type=str, help='task url')
    parser.add_argument('--dry', '-d', action='store_true', help='dry run')
    parser.add_argument('comment', nargs='?', default='', help='comments')

    args = parser.parse_args()
    main(args)