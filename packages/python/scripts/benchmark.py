import timeit

a = timeit.timeit("""
import llama_reader
llama_reader.read_epub("../node/fixtures/basic-v3plus2.epub")
""", number=10000)
b = timeit.timeit("""
from llama_index.readers.file import (
    EpubReader,
)
reader = EpubReader()
reader.load_data("../node/fixtures/basic-v3plus2.epub")
""", number=10000)

print("llama_reader: ", a)
print("llama_index: ", b)
# llama_reader:  12.97605837500305
# llama_index:  32.85046920899185
