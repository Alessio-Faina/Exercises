#include <bits/stdc++.h>

using namespace std;

void miniMaxSum(vector<int> arr) {
    int min = -1;
    int max = 0;
    
    if (arr.size() == 0) {
        cout << 0 << endl << 0 << endl;
        return;
    } else if (arr.size() == 1) {
        cout << arr[0] << endl << arr[0] << endl;
        return;
    }
   
	 	min = arr[0] + arr[1];
    for (int pivot = 0; pivot < arr.size(); pivot++) {
        for (int rounder = 0; rounder < arr.size(); rounder++) {
            if (rounder == pivot) 
                continue;
            if (arr[pivot] + arr[rounder] > max) {
                max = arr[pivot] + arr[rounder];
            }
            if (arr[pivot] + arr[rounder] < max) {
                min = arr[pivot] + arr[rounder];
            }
						cout << "Tried " << pivot << "/" << rounder << " -> " << min << " " << max << endl;
        }
    }
    cout << min << endl << max << endl;
}

int main() {
	vector<int> a = {1, 2, 3, 4, 5};
	miniMaxSum(a);
	return 0;
}
