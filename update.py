#!python3
import os
from os import path
from urllib.parse import urlparse, ParseResult
from codecs import open

def write(src: str, dst: str, url: ParseResult):
    match os.path.splitext(src)[1]:
        case '.rs':
            prefix = '//'
        case '.hs':
            prefix = '--'
        case _:
            raise NotImplementedError(f'Unsupported file type: {src}')
    with open(src, 'r', 'utf8') as fi, open(dst, 'w', 'utf8') as fo:
        fo.write(f'{prefix} {url}\n')
        fo.write(fi.read())

LC = dict()
def leetcode(url: ParseResult):
    _, _, title, *_ = url.path.split('/')
    if title in LC:
        code = LC[title]
    else:
        code = input(f'Code for {title}: ')
        LC[title] = code
    return [
        f'({url.netloc}) {title}',
        path.join(url.netloc, code),
        f'https://{url.netloc}/problems/{title}/'
    ]

def main(args, url: ParseResult):
    match url.netloc:
        case 'leetcode.com':
            msg, dst, url = leetcode(url)
        case _:
            raise NotImplementedError(f'Unsupported URL')

    src = args.src
    dst += path.splitext(src)[1]
    
    if path.exists(dst):
        msg, copy = f'update{msg}', '\033[93mReplace\033[00m'
    else:
        msg, copy = f'add{msg}', '\033[92mCopy\033[00m'

    print(copy, src, '-->', dst)
    if args.dry:
        system = print
        comment = ''
    else:
        comment = f' {input("Comment: ")}'
        system = os.system
        os.makedirs(path.dirname(dst), exist_ok=True)
        write(args.src, dst, url)
    
    system(f'git add {dst}')
    system(f'git commit -m "{msg}{comment}"')
    

if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='Update submissions')
    parser.add_argument('src', type=str, help='source file of the solution')
    parser.add_argument('--dry', '-d', action='store_true', help='dry run')
    args = parser.parse_args()
    s = ''
    while (inp := input('New URL (. to exit): ')) != '.':
        s = inp.strip() or s
        main(args, urlparse(s))
        print('\nEnter to use ', s)
