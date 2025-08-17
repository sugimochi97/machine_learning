# FizzBuzz プログラム
# 1から100までの数字について：
# 3の倍数のときは"Fizz"
# 5の倍数のときは"Buzz"  
# 15の倍数のときは"FizzBuzz"
# それ以外はその数字を出力

for i in range(1, 100):
    if i % 15 == 0:
        print("FizzBuzz")
    elif i % 3 == 0:
        print("Fizz")
    elif i % 5 == 0:
        print("Buzz")
    else:
        print(i)
