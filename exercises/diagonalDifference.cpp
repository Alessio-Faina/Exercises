#include <bits/stdc++.h>

using namespace std;

int diagonalDifference(vector<vector<int>> arr) {
    int leftDiag = 0;
    int rightDiag = 0;
    for (int i = 0, j = 0; i < arr.size(); i++, j++) {
        leftDiag += arr[i][j];
    }
    for (int i = arr.size() - 1, j = 0; j < arr.size(); i--, j++) {
				cout << arr[i][j] << endl; 
        rightDiag += arr[i][j];
    }
   	cout << leftDiag << " - " << rightDiag << endl; 
    return std::abs(leftDiag - rightDiag);
}

int main() {

	vector<vector<int>> arr = {{11, 2, 4}, {4, 5, 6},{10, 8, -12}};
	int res = diagonalDifference(arr);
	cout << res;
	return 0;
}
