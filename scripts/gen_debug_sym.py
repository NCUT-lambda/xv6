#!/usr/bin/env python3


def process_file(file_name: str):
    xs = []
    with open(file_name, "r") as f:
        xs = f.readlines()
        xs = list(map(lambda x: x.split(" ")[1], xs))
        xs = sorted(xs)
    with open(file_name, "w") as f:
        f.writelines(xs)


def main():
    process_file("debug_c.sym")
    process_file("debug_rs.sym")


if __name__ == "__main__":
    main()
