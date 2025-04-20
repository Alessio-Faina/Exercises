#include <cmath>
#include <iomanip>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

void flipColumn(vector<vector<int>> &matrix, int subMatrixSize, int column) {
    int needFlip = 0;
    int maxNumber = -1;
    int maxNumberIndex = -1;
    for (int i = 0; i < matrix.size(); i++) {
        if (maxNumber < matrix[i][column]) {
            maxNumber = matrix[i][column];
            maxNumberIndex = i;
        }
    }
    if (maxNumberIndex >= subMatrixSize) {
        needFlip = 1;
    }

    if (needFlip) {
        for (int i = 0, j = matrix.size() - 1; i < subMatrixSize; i++, j--) {
            int temp = matrix[i][column];
            matrix[i][column] = matrix[j][column];
            matrix[j][column] = temp;
        }
    }
}

void flipRow(vector<vector<int>> &matrix, int subMatrixSize, int row) {
    int needFlip = 0;
    int maxNumber = -1;
    int maxNumberIndex = -1;
    for (int i = 0; i < matrix.size(); i++) {
        if (maxNumber < matrix[row][i]) {
            maxNumber = matrix[row][i];
            maxNumberIndex = i;
        }
    }
    if (maxNumberIndex >= subMatrixSize) {
        needFlip = 1;
    }

    if (needFlip) {
        for (int i = 0, j = matrix.size() - 1; i < subMatrixSize; i++, j--) {
            int temp = matrix[row][i];
            matrix[row][i] = matrix[row][j];
            matrix[row][j] = temp;
        }
    }
}

void debugPrint(vector<vector<int>> matrix) {
    for (int i = 0; i < matrix.size(); i++) {
        for (int j = 0; j < matrix.size(); j++) {
            cout << std::setw(3) << matrix[i][j] << " ";
        }
        cout << endl;
    }
		cout << endl;
}

int flippingMatrix_NO(vector<vector<int>> matrix) {
		debugPrint(matrix);
    int subMatrixSize = matrix.size() / 2;
    for (int column = 0; column < matrix.size(); column++) {
        flipColumn(matrix, subMatrixSize, column);
    }
		debugPrint(matrix);
    for (int row = 0; row < matrix.size(); row++) {
        flipRow(matrix, subMatrixSize, row);
    }
		debugPrint(matrix);

    int result = 0;
    for (int row = 0; row < subMatrixSize; row++) {
        for (int column = 0; column < subMatrixSize; column++) {
            result += matrix[row][column];
        }
    }

    return result;
}

int flippingMatrix(vector<vector<int>> matrix) {
	int result = 0;
	int subMatrixSize = matrix.size() / 2;
	int s = matrix.size() - 1;
	for (int i = 0; i < subMatrixSize; i++) {
		for (int j = 0; j < subMatrixSize; j++) {
			result += max(
								max(matrix[i][j], matrix[s - i][j]) ,
						    max(matrix[i][s - j], matrix[s - i][s - j])
					);
		}
	}

	return result;
}

int main() {

	vector<vector<int>> matrix_one = {	{112, 42, 83, 119}, 
																		{56, 125, 56, 49},
																		{15, 78, 101, 43},
																		{62, 98, 114, 108}};
  //expected 414
	vector<vector<int>> matrix_two = {	{107, 54, 128, 15},
																			{12, 75, 110, 138},
																			{100, 96, 34, 85},
																			{75, 15, 28, 112}};
	//expected 488
	cout << flippingMatrix(matrix_one) << endl << endl;
	cout << flippingMatrix(matrix_two) << endl;
	return 0;
}
