import tkinter
import tkinter.filedialog

import PIL.Image

import colors
from data import Data


class App(tkinter.Frame):
    image: PIL.Image.Image
    data_image: Data
    canvas: tkinter.Canvas

    def __init__(self, master=None):
        super().__init__(master)
        self.master = master
        self.pack()
        self.create_widgets()

    def create_widgets(self):
        self.canvas = tkinter.Canvas()
        self.canvas.pack()

        menubar = tkinter.Menu(self)

        file_menu = tkinter.Menu(menubar)
        file_menu.add_command(label='Открыть', command=self.open_file())
        file_menu.add_command(label='Сохранить', command=save_file(self))

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

        self.image = PIL.Image.open(file)
        self.data_image = Data.from_image(self.image)


def save_file(app: App):
    path = tkinter.filedialog.asksaveasfilename()

    app.image.save(path)


def perform(app: App, func):
    app.__data_image = func(app.data_image)
    app.__image = app.data_image.to_image()
