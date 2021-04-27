import tkinter
from app import App

def main():
    root = tkinter.Tk()
    root.geometry("300x200")

    app = App(master=root)
    app.mainloop()

if __name__ == "__main__":
    main()
