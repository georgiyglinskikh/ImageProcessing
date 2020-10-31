PROJECT_NAME = ImageProcessingCPP

C_CPP = clang++

INCLUDE_PATH = -I /usr/local/include/
LIBS_PATH = -L /usr/local/lib/

LIBS = -lOpenImageIO

SOURCES = ./src/main.cpp #./src/*.{cpp,hpp}
BINARY = ./bin/$(PROJECT_NAME)

run:
	$(C_CPP) $(SOURCES) -o $(BINARY) -O3 $(INCLUDE_PATH) $(LIBS_PATH) $(LIBS)