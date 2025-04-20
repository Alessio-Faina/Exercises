#include <bits/stdc++.h>

using namespace std;

long flippingBits(long n) {
		uint32_t result = static_cast<uint32_t>(n);
    n = ~result;
    return n;
    
		for (int i = 0; i < 32; i++) {
			int res = (n & (1 << i));
			res ? cout << "1" : cout<<"0";
		}
		cout << endl;

		for (int i = 0; i < 32; i++) {
        if (result & (1 << i)) {
            result &= ~(1 << i);
        } else {
            result |= (1 << i);
        }
//				cout << i << " - " << result << endl;
    }

		for (int i = 0; i < 32; i++) {
			int res = (result & (1 << i));
			res ? cout << "1" : cout<<"0";
		}
		cout << endl;

		return result;
}

int main() {
	int res = flippingBits(3);
	cout << endl << res << endl << endl;
	res = flippingBits(2147483647);
	cout << endl << res << endl << endl;
	res = flippingBits(1);
	cout << endl << res << endl << endl;
	res = flippingBits(0);
	cout << endl << res << endl << endl;
	return 0;
}
