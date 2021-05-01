import numpy as np

from args_parser import parse_args
from process import process
from utils import Data


def main():
    args = parse_args()

    data = Data(np.array([]), args)
    data.open_image()

    data = process(data)

    data.save_image()


if __name__ == '__main__':
    main()
