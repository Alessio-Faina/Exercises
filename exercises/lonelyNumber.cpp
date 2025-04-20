#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);
vector<string> split(const string &);

int BITS_lonelyinteger(vector<int> a) {
    unsigned int lowerInts = 0;
    unsigned int higherInts = 0;
    int result = -1;

		if (a.size() == 1) {
        return a[0];
    }

    for (int i = 0; i < a.size(); i++) {
        if (a[i] < 64) {
            lowerInts |= (1 << a[i]);
						cout << "Setting bit " << a[i] << endl;
        } else {
            higherInts |= (1 << (a[i] - 64));
						cout << "Setting bit " << a[i] << endl;
        }
    }

    for (int i = 0; i < a.size(); i++) {
        if (i < 64) {
            if (! ( lowerInts & (1 << i))) {
								cout << "Found bit " << i << endl;
                result = i;
                break;
            }
        } else {
            if (! (higherInts & (1 << (i - 64)))) {
								cout << "Found bit " << i << endl;
                result = i;
                break;
            }
        }
    }
    return result;
}


int lonelyinteger(vector<int> a) {
    if (a.size() == 1) {
        return a[0];
    }
    int result = -1;
    std::sort (a.begin(), a.end());

		for (int i = 0; i < a.size(); i++) {
			cout << a[i] << ", ";
		}
    for (int i = 0; i < a.size(); i+=2) {
        if (!(a[i + 1] == a[i])) {
            result = a[i];
						break;
        }
    }
		if (result == -1) {
			result = a[a.size()-1];
		}
    return result;
}


int main() {
	vector<int> a = {18, 49, 15, 30, 56, 20, 49, 67, 82, 69, 84, 63, 93, 87, 66, 17, 38, 32, 17, 32, 94, 66, 67, 63, 48, 64, 84, 82, 87, 18, 79, 64, 79, 15, 71, 94, 59, 5, 22, 59, 4, 98, 81, 4, 98, 73, 69, 88, 30, 81, 48, 56, 71, 20, 93, 22, 73, 5, 88};
	int res = lonelyinteger(a);
	cout << endl << res;
	return 0;
}
