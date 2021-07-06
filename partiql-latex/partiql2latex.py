import argparse as _argparse


def main():
    parser = _argparse.ArgumentParser(
        description='Converts Ion representation of the PartiQL Pest Grammar to LaTex files'
    )
    parser.add_argument('--output')


if __name__ == '__main__':
    main()