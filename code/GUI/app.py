from data import Data
import PIL.Image, tkinter, tkinter.filedialog

import colors

class App(tkinter.Frame):
    __image: PIL.Image.Image
    __data_image: Data
    __canvas: tkinter.Canvas

    def __init__(self, master=None):
        super().__init__(master)
        self.master = master
        self.pack()
        self.create_widgets()

    def create_widgets(self):
        self.__canvas = tkinter.Canvas()
        self.__canvas.pack()

        menubar = tkinter.Menu(self)

        file_menu = tkinter.Menu(menubar)
        file_menu.add_command(label='Открыть', command=self.open_file)
        file_menu.add_command(label='Сохранить', command=self.save_file)

        color_menu = tkinter.Menu(menubar)
        color_menu.add_command(label='Преобразовать в черно/белое', command=self.perform(colors.make_white_black))

        menubar.add_cascade(label='Файл', menu=file_menu)
        menubar.add_cascade(label='Цвет', menu=color_menu)

        self.master.config(menu=menubar)

    def open_file(self):
        file = tkinter.filedialog.askopenfilename(
            title='Открыть файл',
            initialdir='.',
            filetypes=[('bmp', '*.bmp'), ('jpg', '*.jpg'), ('png', '*.png')])

        self.__image = PIL.Image.open(file)
        self.__data_image = Data.from_image(self.__image)

    def save_file(self):
        path = tkinter.filedialog.asksaveasfilename()

        self.__image.save(path)

    def perform(self, func):
        self.__data_image = func(self.__data_image)
        self.__image = PIL.Image.fromarray(self.__data_image, mode='RGB')
