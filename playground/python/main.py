class MyClass:
    """A simple example class"""

    i = 12345

    def ini(self):
        x.i = 54321
        self.name = "Chara-X"
        self.age = 20

    def new(name, age):
        x = MyClass()
        x.name = name
        x.age = age
        return x

    def f(self):
        """This function returns a string "hello world"""
        print(self.i)
        return "hello world"


x = MyClass()
print(x.i)
print(x.f())
print(x.__doc__)
