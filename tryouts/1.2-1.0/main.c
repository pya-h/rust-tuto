#include <stdio.h>

int main() {
	double a = 1.2, b = 1.0, c = -1.0;
	double u = 2.6, v = 2.0;
	printf("By double: %.16lf\t%.16lf\n", a + b * c, u + c * v);

	float af = 1.2f, bf = 1.0f, cf = -1.0f;
	float uf = 2.6f, vf = 2.0f;
	printf("\tBy float: %.16f\t%.16f", af + bf * cf, uf + cf * vf);
}
