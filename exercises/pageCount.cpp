#include <cmath>
#include <iomanip>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;


int pageCount(int n, int p) {
    if (n == 1 || n == p) return 0;
		if ((n % 2 == 0) && (p == n-1)) return 1;

    int flips = 0;
    if (p > n/2) {
            flips = (n-p)/2;
        } else {
            flips = p/2;
    }
    cout << "Flip = " << flips << endl;
    if (p % n == 0) {
        flips++;
        cout << "Decreasing" << endl;
    }
    return flips;
}

int main() {
	cout << pageCount(8, 7) << endl;
	return 0;
}
