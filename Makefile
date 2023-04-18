##
## EPITECH PROJECT, 2023
## B-OOP-400-MAR-4-1-raytracer-erwan.gonzales
## File description:
## Makefile
##

TARGET = raytracer

all:
	cargo build
	cp target/debug/$(TARGET) $(TARGET)

clean:
	cargo clean

fclean: clean
	rm -f $(TARGET)

re: fclean all
