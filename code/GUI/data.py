from PIL.Image import Image
import PIL.Image
import numpy as np


class Data:
    data: np.ndarray

    def __init__(self, data: np.ndarray):
        self.data = data

    @staticmethod
    def from_image(image: Image):
        return Data(np.array(image))

    @classmethod
    def to_image(cls) -> Image:
        image = PIL.Image.fromarray(cls.data, mode='RGBA')

        return image
