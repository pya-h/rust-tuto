#include <stdio.h>

int main() {
	double a = 1.2, b = 1.0, c = -1.0;
	float u = 2.6f, v = 2.0f;
	printf("%.16f\t%.16lf", a + b * c, u + c * v);
}
