from PIL import Image
import numpy as np


class Data:
    data: np.ndarray
    args: any

    def __init__(self, data: np.ndarray, args: any):
        self.data = data
        self.args = args

    def apply_to(self, func):
        """Применить функцию ко всем элементам массива"""
        vect_func = np.vectorize(func)

        return Data(vect_func(self.data), self.args)

    def apply_to_array(self, func, dimension=-1) -> np.ndarray:
        """Применить функцию с массиву с ```dimension``` измерений"""
        return Data(np.apply_along_axis(func, dimension, self.data), self.args)

    def get_subdata(self, x: int, y: int, R: int):
        """Взятие квадрата с данными по координатам ```x, y``` со стороной ```R```"""

        # "Радиус" квадрата
        half_R = R // 2

        # Срез по +-радиус квадрата <=> квадрат со стороной R
        subdata = Data(self.data[(
            x - half_R):(x + half_R + 1), (y - half_R):(y + half_R + 1)], self.args)

        return subdata

    def filter_data(self, func):
        """Линейно фильтруем данные ```data```, \
        размер промежуточной матрицы - ```R```, \
        с помощью пользовательской функции ```func```"""

        # Теперь у нас R будет строго нечетным (середина + 2 половинки)
        R = self.args.R
        R = R - R % 2 + 1
        half_R = R // 2  # Половинка

        # Заполняем края нулями
        data_0 = np.pad(self.data, pad_width=half_R,
                        mode="constant", constant_values=0)

        # Применяем функцию -> Преобразуем в список -> Преобразуем в np.ndarray
        filtered_array: np.ndarray = np.array(list(map(  # Преобразования типов
            lambda info:  # Функция, обрабатывающая данный
            int(func(self.get_subdata(data_0,  # Приминяем пользовательскую функцию к данным
                                      info[0][0] + half_R,  # X
                                      info[0][1] + half_R, R))),  # Y, R
            np.ndenumerate(  # Составляем массив индекс-значение
                data_0
                [half_R:-half_R,
                 half_R:-half_R]))))

        # Получаем размеры изображения
        size = self.data.shape[0], self.data.shape[1]

        # Преобразуем изображение в известный нам размер
        filtered_data = np.reshape(filtered_array, size)

        filtered_data_obj = Data(filtered_data, self.args)

        return filtered_data_obj

    def norm(self):
        Data((self.data / np.max(self.data)).astype(np.uint0), self.args)

    def open_image(self):
        image = Image.open(self.args.i)

        self.data = np.array(image)

    def save_image(self):
        image = Image.fromarray(self.data.data, 'RGBA')

        image.save(self.args.o)
