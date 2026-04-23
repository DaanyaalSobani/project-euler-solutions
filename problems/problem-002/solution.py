import sys
def fibanchi_gen():
    a,b = 0,1
    while True:
        yield a
        a,b = b,a+b

def main():
    cutoff = int(sys.argv[1]) if len(sys.argv) > 1 else 4_000_000
    total = 0
    for n in fibanchi_gen():
        if n > cutoff:
            break
        if n % 2 == 0:
            total += n
    print(total)

if __name__ == "__main__":
    main()