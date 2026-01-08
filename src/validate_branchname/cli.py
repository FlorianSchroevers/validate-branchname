import argparse
import re
import subprocess
import sys


def run_git(*args):
    """ Run a git command with given args using subprocess """
    process = subprocess.run(
        ["git"] + list(args),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )

    return process.stdout.strip(), process.stderr.strip(), process.returncode

def current_branch_name() -> str:
    """
    Determine and return the current branchname

    :return: branchname
    :rtype: str
    """
    out, err, code = run_git("rev-parse", "--abbrev-ref", "HEAD")

    if code != 0:
        raise RuntimeError(f"git error: {err or out or 'unknown error'}")

    if out == "HEAD":
        raise RuntimeError(f"head detached error: please checkout a branch.")

    return out

def validate_branchname(pattern: str) -> bool:
    """
    Return true if the given branch matches the given regex pattern
    
    :param pattern: regex pattern
    :type pattern: str
    :return: Whether the pattern matches the current branch
    :rtype: bool
    """
    try:
        pattern_compiled = re.compile(pattern)
    except re.error as e:
        raise RuntimeError(f"regex error: Invalid pattern {e}")
    
    try:
        name = current_branch_name()
    except RuntimeError as e:
        raise RuntimeError(f"Error getting branch name: {e}")

    return pattern_compiled.search(name) is not None


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="validate_branchname",
        description="Match current branch name with given regex pattern",
        usage="validate_branchname --pattern <regex_pattern>"
    )

    parser.add_argument(
        "-p", "--pattern",
        required=True,
        help="Regex pattern to match against the current branch name"
    )

    return parser.parse_args()

def main() -> None:
    args = parse_args()

    try:
        matched = validate_branchname(args.pattern)
    except RuntimeError as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
    
    sys.exit(0 if matched else 1)

if __name__ == "__main__":
    main()
