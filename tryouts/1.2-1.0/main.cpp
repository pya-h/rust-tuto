#include <iostream>

int main() {
	double a = 1.2, b = 1.0, c = -1.0;
	double u = 2.6, v = 2.0;
	std::cout << "By double: "<< a + b * c << "\t" << u + c * v<< std::endl;

	float af = 1.2f, bf = 1.0f, cf = -1.0f;
	float uf = 2.6f, vf = 2.0f;
	std::cout << "\tBy float: " << af + bf * cf << "\t" << uf + cf * vf;
}
