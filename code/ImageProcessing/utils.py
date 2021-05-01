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
        vector_func = np.vectorize(func)

        return Data(vector_func(self.data), self.args)

    def apply_to_array(self, func, dimension=-1):
        """Применить функцию с массиву с ```dimension``` измерений"""
        return Data(np.apply_along_axis(func, dimension, self.data), self.args)

    def get_subdata(self, x: int, y: int, r: int):
        """Взятие квадрата с данными по координатам ```x, y``` со стороной ```R```"""

        # "Радиус" квадрата
        half_r = r // 2

        # Срез по +-радиус квадрата <=> квадрат со стороной R
        subdata = Data(self.data[(
            x - half_r):(x + half_r + 1), (y - half_r):(y + half_r + 1)], self.args)

        return subdata

    def filter_data(self, func):
        """Линейно фильтруем данные ```data```, \
        размер промежуточной матрицы - ```r```, \
        с помощью пользовательской функции ```func```"""

        # Теперь у нас r будет строго нечетным (середина + 2 половинки)
        r = self.args.R
        r = r - r % 2 + 1
        half_r = r // 2  # Половинка

        # Заполняем края нулями
        data_0 = Data(np.pad(self.data, pad_width=half_r,
                             mode="constant", constant_values=0), self.args)

        # Применяем функцию -> Преобразуем в список -> Преобразуем в np.ndarray
        filtered_array: np.ndarray = np.array(list(map(  # Преобразования типов
            lambda info:  # Функция, обрабатывающая данный
            int(func(data_0.get_subdata(  # Применяем пользовательскую функцию к данным
                info[0][0] + half_r,  # X
                info[0][1] + half_r, r).data)),  # Y, r
            np.ndenumerate(  # Составляем массив индекс-значение
                data_0.data
                [half_r:-half_r,
                 half_r:-half_r]))))

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
        image = Image.fromarray(self.data, mode='RGBA')

        image.save(self.args.o)
