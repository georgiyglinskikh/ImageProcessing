# import typing # Статическая типизация
# import IPython.display # Вывод изображений сюда

import numpy as np # Обработка массивов
from PIL import Image # Загрузка изображений с диска, преобразование массива в картинку
import math # Простые математические операторы

image = Image.open("../images/floor.bmp") # Считываем картинку в переменную

data: np.ndarray = np.array(image) # Преобразовываем картинку в двумерный массив с тройками чисел, обозначающими цвета

def apply_to(array: np.ndarray, func) -> np.ndarray:
    """Применить функцию ко всем элементам массива"""
    vect_func = np.vectorize(func)
    return vect_func(array)

def apply_to_array(array: np.ndarray, func, dimension=-1) -> np.ndarray:
    """Применить функцию с массиву с ```dimension``` измерений"""
    return np.apply_along_axis(func, dimension, array)

c1_2 = 0.299
c2_2 = 0.587
c3_2 = 0.114

wb_2_2: np.ndarray = apply_to_array(data, lambda x: int(c1_2 * x[0] + c2_2 * x[1] + c3_2 * x[2]))
"""Ч/Б изображение, полученное с помощью умножения каналов на константы ```cn_2```"""

data_wb: np.ndarray = wb_2_2
"""Итоговая картинка черно-белого изображения"""

def get_subdata(data: np.ndarray, x: int, y: int, R: int) -> np.ndarray:
    """Взятие квадрата с данными по координатам ```x, y``` со стороной ```R```"""

    # "Радиус" квадрата
    half_R = R // 2

    # Срез по +-радиус квадрата <=> квадрат со стороной R
    return data[(x - half_R):(x + half_R + 1), (y - half_R):(y + half_R + 1)]

def filter_data(data: np.ndarray, R: int, func) -> np.ndarray:
    """Линейно фильтруем данные ```data```,     размер промежуточной матрицы - ```R```,     с помощью пользовательской функции ```func```"""

    R = R - R % 2 + 1 # Теперь у нас R будет строго нечетным (середина + 2 половинки)
    half_R = R // 2 # Половинка

    # Заполняем края нулями
    data_0 = np.pad(data, pad_width=half_R, mode="constant", constant_values=0)

    # Применяем функцию -> Преобразуем в список -> Преобразуем в np.ndarray
    filtered_array: np.ndarray = np.array(list(map( # Преобразования типов
        lambda info: # Функция, обрабатывающая данный
            int(func(get_subdata(data_0, # Приминяем пользовательскую функцию к данным
                    info[0][0] + half_R, # X
                    info[0][1] + half_R, R))), # Y, R
        np.ndenumerate( # Составляем массив индекс-значение
            data_0
                [half_R:-half_R,
                half_R:-half_R]))))

    # Получаем размеры изображения
    size = data.shape[0], data.shape[1]

     # Преобразуем изображение в известный нам размер
    filtered_data = np.reshape(filtered_array, size)

    return filtered_data

data_fitered: np.ndarray = filter_data(data_wb, 3, lambda x: np.median(x))
"""Итоговое фильтрованное изображение"""

# def normalize(data: np.ndarray, c: int) -> np.ndarray:
#     """Преобразование черно-белого изображения из 8битового в 1битное"""
#
#     # Функция сверения с порогом ```с```
#     f = lambda x: 0 if x <= c else 1
#
#     # Попиесельное применение к ```data```
#     return apply_to(data.astype(np.uint0), f)
#
# norm_2 = normalize(data_fitered, np.median(data_fitered))
# """1битное изображение, полученное по медиане"""

data_normed: np.ndarray = data_fitered  # norm_2
"""Итоговое изображение после нормализации"""

sobel_matrix: np.ndarray = np.array([
    [1,   2,  1],
    [0,   0,  0],
    [-1, -2, -1]])
"""Матрица со специальными значениями для фильтра"""

sobel_matrix_T: np.ndarray = sobel_matrix.transpose()
"""Транспонированая матрица фильтра Собеля"""

def sobel_filter(data: np.ndarray) -> int:
    """Фильтруем фрагмент (```data```) с помощьб фильтра собеля"""

    Gx: int = np.sum(data * sobel_matrix) # Изменение по X
    Gy: int = np.sum(data * sobel_matrix_T) # Изменение по Y

    return int(math.hypot(Gx, Gy)) # По теореме пифагора находим результат

data_sobel: np.ndarray = filter_data(data_normed, 3, sobel_filter)

data_sobel_norm: np.ndarray = (data_sobel / np.max(data_sobel)).astype(np.uint0)

res = Image.fromarray(data_sobel_norm, "1")

res.save("./res.bmp")
