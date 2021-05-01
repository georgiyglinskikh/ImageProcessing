import numpy as np

from utils import Data


def make_white_black(data: Data, c: np.ndarray) -> Data:
    return Data(data.apply_to_array(lambda x: np.sum(x * c)), data.args)


def change_channel(data: Data, c: np.ndarray) -> Data:
    return Data(data.apply_to(lambda x: x * c), data.args)
