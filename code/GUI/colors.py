import numpy as np

from data import Data


def make_white_black(data: Data, coofs: np.ndarray) -> Data:
    return Data(data.apply_to_array(lambda x: np.sum(x * coofs)), data.args)


def change_channel(data: Data, coofs: np.ndarray) -> Data:
    return Data(data.apply_to(lambda x: x * coofs), data.args)
