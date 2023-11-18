#include <emscripten/emscripten.h>
#include <stdint.h>


EMSCRIPTEN_KEEPALIVE
double sumFloat64Array(const double* array, int length) {
    double sum = 0.0;
    for (int i = 0; i < length; i++) {
        sum += array[i];
    }
    return sum;
}

EMSCRIPTEN_KEEPALIVE
uint32_t sumArray(uint8_t* array, int length) {
    uint32_t sum = 0;
    for (int i = 0; i < length; i++) {
        sum += array[i];
    }
    return sum;
}

EMSCRIPTEN_KEEPALIVE
int square(int n) {
    return n * n;
}