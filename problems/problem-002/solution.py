def fibanchi_gen():
    a,b = 0,1
    while True:
        yield a
        a,b = b,a+b

def main():
    cutoff = 4_000_000
    sum = 0
    for n in fibanchi_gen():

        if n > cutoff:
            break

        if n % 2 == 0:
            sum += n

    print(sum)

if __name__ == "__main__":
    main()