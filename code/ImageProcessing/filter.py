import numpy as np
from utils import Data
import math

def func(is_avg: bool):
    if is_avg:
        return lambda x: x.mean()

    return lambda x: np.median(x)

def filter(data: Data):
    data.filter_data(func(data.args.avg))

def sobel_filter(data: Data):
    sobel_matrix: np.ndarray = np.array([
        [1,   2,  1],
        [0,   0,  0],
        [-1, -2, -1]])


    sobel_matrix_T: np.ndarray = sobel_matrix.transpose()

    def sobel_filter(data: np.ndarray) -> int:
        """Фильтруем фрагмент (```data```) с помощьб фильтра собеля"""

        Gx: int = np.sum(data * sobel_matrix) # Изменение по X
        Gy: int = np.sum(data * sobel_matrix_T) # Изменение по Y

        return int(math.hypot(Gx, Gy)) # По теореме пифагора находим результат

    data_sobel = data.filter_data(sobel_filter)

    result = data_sobel.norm()
