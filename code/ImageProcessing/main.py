from utils import Data
from args_parser import parse_args
from process import process
import numpy as np

def main():
    args = parse_args()

    data = Data(np.array([]), args)
    data.open_image()

    data = process(data)

    data.save_image()


if __name__ == '__main__':
    main()
