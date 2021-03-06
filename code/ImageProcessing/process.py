import numpy as np
from utils import Data
from colors import make_white_black
from filter import filter_data, sobel_filter


def process(data: Data) -> Data:
    result = data

    if result.args.white_black:
        result = make_white_black(result, np.array([0.299, 0.587, 0.114, 1]))

    if result.args.filter:
        result = filter_data(result)

    if result.args.bit:
        result.norm()

    if result.args.sobel:
        result = sobel_filter(data)

    return result
