from math import tan, radians
from cfonts import render

output = render('Portal Finder', colors=['green', 'blue'], align='center')
print(output)

try:
    x1 = float(input('Введите x1: '))
    y1 = float(input('Введите y1: '))
    a = float(input('Введите угол a: '))

    x2 = float(input('Введите x2: '))
    y2 = float(input('Введите y2: '))
    b = float(input('Введите угол b: '))

    k1 = tan(radians(a+90))  # tan (90+a)
    k2 = tan(radians(b+90))  # tan (90+b)

    b1 = y1-k1*x1
    b2 = y2-k2*x2

    print(f"\ny={k1}x{'+'+str(b1) if b1>0 else str(b1)}")
    print(f"y={k2}x{'+'+str(b2) if b2>0 else str(b2)}")

    d = -1*k1+k2
    d_x = b1-b2
    d_y = k1*(-1)*b2+b1*k2

    final_x = d_x/d
    final_y = d_y/d

    print(f"\nРезультат: {final_x} ~ {final_y}")
    output = render('FINISHED', colors=['green', 'yellow'], align='center')
    print(output)
    input()
except ValueError:
    fail = render('FAIL', colors=['red', 'yellow'], align='center')
    print(fail)
    print('пошел нах*й')
    input()
except ZeroDivisionError:
    fail = render('FAIL', colors=['red', 'yellow'], align='center')
    print(fail)
    print('Во время работы было деление на ноль')
    input()
except Exception:
    fail = render('FAIL', colors=['red', 'yellow'], align='center')
    print(fail)
    print('Что то пошло не так')
    input()
