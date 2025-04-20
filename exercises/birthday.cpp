#include <bits/stdc++.h>

using namespace std;

int birthday(vector<int> s, int d, int m) {
		int result = 0;
		cout << s.size() << endl;
		if (s.size() == 1) {
			if (s[0] == d) {
				return 1;
			}
		}
    for (int i = 0; i < s.size() - m + 1; i++) {
        int sum = 0;
        for (int j = i; j < i + m; j++) {
            sum+=s[j];
        }
				cout << i << "->" << i+m << "= " << sum << endl;
        if (sum == d) {
            result++;
        }
    }
    return result;
}

int main() {
	vector<int> s = {3, 5, 4, 1, 2, 5, 3, 4, 3, 2, 1, 1, 2, 4, 2, 3, 4, 5, 3, 1, 2, 5, 4, 5, 4, 1, 1, 5, 3, 1, 4, 5, 2, 3, 2, 5, 2, 5, 2, 2, 1, 5, 3, 2, 5, 1, 2, 4, 3, 1, 5, 1, 3, 3, 5};
	int d = 18;
	int m = 6;
	cout << birthday(s, d, m) << endl;
	/*
	vector<int> s = {1,2,1,3,2};
	int d = 3;
	int m = 2;
	cout << birthday(s, d, m) << endl;

	s = {4};
	d = 4;
	m = 1;
	cout << birthday(s, d, m) << endl;
*/
	return 0;
}
