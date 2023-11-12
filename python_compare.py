import numpy as np
import time
t = time.time()

with open("./a_n_u16__random", "rb") as file:
    byte_data = file.read()

    # print(byte_data)
    # Convert the byte data to a numpy array of uint8
    uint8_array = np.frombuffer(byte_data, dtype=np.uint8)
    # Convert the numpy array of uint8 to uint16
    # Assuming the byte order is little-endian; adjust as needed
    uint16_array = uint8_array.view(np.uint16)

    m = np.median(uint16_array)
    print(m)
    print('median')
    print(time.time()-t)

