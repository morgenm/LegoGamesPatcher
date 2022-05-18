if __name__ == "__main__":
    # Gen SW TSS binary
    with open("tss.exe", "wb") as f:
        arr = []
        for i in range(1024):
            arr.append(0)

        arr += [0,1,0,0,15,133, 137, 0]
        arr += [0,0,0,0,0,0,0,0]
        b = bytearray(arr)
        f.write(b)

    # Gen older binary
    with open("older.exe", "wb") as f:
        arr = []
        for i in range(1024):
            arr.append(0)

        arr += [116, 11, 185, 1]
        for i in range(1024):
            arr.append(0)
        b = bytearray(arr)
        f.write(b)