import json
import random

def randomBool():
    return random.choice([True, False])

def randomNum(start, end):
    return random.randint(start, end)

def randomNumber():
    return randomNum(0, 2**31-1)

def randomChar():
    return chr(randomNum(0, 255))

def randomString():
    str_len = randomNum(0, 100)
    return "".join(randomChar() for _ in range(str_len))

def randomElem():
    choice = randomNum(1, 91)
    if choice <= 40:
        return randomBool()
    elif choice <= 80:
        return randomNumber()
    elif choice <= 90:
        return randomString()
    elif choice == 91:
        return randomList()

def randomList():
    lst_len = randomNum(0, 100)
    return list(randomElem() for _ in range(lst_len))

def randomObject(base, power):
    if power == 0:
        return randomElem()

    obj = {}
    for i in range(base):
        obj[f"key_{i}_{power}"] = randomObject(base, power-1)
    return obj

def main():
    obj = randomObject(2, 3)
    print(json.dumps(obj))

if __name__ == '__main__':
    main()