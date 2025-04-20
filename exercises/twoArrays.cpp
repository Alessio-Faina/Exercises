#include <bits/stdc++.h>

using namespace std;

string twoArrays_TOOSLOW(int k, vector<int> A, vector<int> B) {
    do {
        int i = 0;
        for (; i < B.size(); i++) {
            if ((A[i] + B[i]) < k) {
                break;
            }
        }
        if (i == B.size()) {
            return "YES";
        }
    } while (std::next_permutation(A.begin(), A.end()));
    return "NO";
}

string twoArrays(int k, vector<int> A, vector<int> B) {
    std::sort(A.begin(), A.end());
    std::sort(B.begin(), B.end(), std::greater<>());

    for (int i = 0; i < A.size(); i++) {
        if ((A[i] + B[i]) < k) {
            return "NO";
        }
    }
    return "YES";
}

int main() {
	int k = 10;
	vector<int> A = {2, 1, 3};
	vector<int> B = {7, 8, 9};
	cout << twoArrays(k, A, B);

	k = 5;
	A = {1, 2, 2, 1};
	B = {3, 3, 3, 4};
	cout << twoArrays(k, A, B);

	return 0;
}
