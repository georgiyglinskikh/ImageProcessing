import argparse

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description='Make some effects to image')

    return parser


def add_arguments(parser: argparse.ArgumentParser):
    parser.add_argument('-i', type=str, help='Path to file')

    parser.add_argument('-o', type=str, help='Path to output file')

    parser.add_argument('-avg', action='store_true',
                        help='Should we use average funtion instead of median')

    parser.add_argument('--whiteblack', action='store_true',
                        help="Make picture white and black")

    parser.add_argument('--filter', action='store_true',
                        help='Make picture less noisy with given radius')

    parser.add_argument('--bit', action='store_true',
                        help='Transform picture to 0 and 1')

    parser.add_argument('--sobel', action='store_true', help='Select borders of image')

    parser.add_argument('--resize', type=float,
                        help='Resize image in n times (to make bigger - n < 1)')

    parser.add_argument('--rotate', type=int,
                        help='Rotate image n times clockwise')

    parser.add_argument('--r', type=float, help='Strenght of RED channel')
    parser.add_argument('--g', type=float, help='Strenght of GREEN channel')
    parser.add_argument('--b', type=float, help='Strenght of BLUE channel')

    parser.add_argument('-R', type=int, default=3, help='Radius for filters')


def parse_args():
    parser = build_parser()

    add_arguments(parser)

    args = parser.parse_args()

    return args
