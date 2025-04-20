#include <cmath>
#include <iomanip>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

int maxMin(int k, vector<int> arr) {
    std::sort(arr.begin(), arr.end());
    int unfairness = -1;
    for (int i = 0; i <= arr.size() - k; i++) {
        int maxInt = arr[i + k - 1];
        int minInt = arr[i];
        int temp = maxInt - minInt;
				cout << " ---> " << minInt << " " << maxInt << " | " << maxInt - minInt << " || ";
        if (unfairness == -1) unfairness = temp;
        else if (temp < unfairness) unfairness = temp;
				cout << unfairness << endl;
    }

    return unfairness;
}


int main() {
	int k = 25;
	vector<int> arr ={
1399, 6925, 3074, 6733, 6608, 6919, 8712, 8438, 
8221, 3352, 8493, 1417, 2157, 1798, 7964, 5543, 
2924, 9261, 5280, 781, 8314, 7891, 5900, 4578, 
1908, 1593, 7272, 7497, 9059, 8763, 7724, 6810, 
2040, 7150, 3543, 8648, 421, 8607, 3438, 8642, 
8311, 8284, 6412, 468, 6434, 728, 2363, 9359, 
9989, 7643, 6492, 4656, 5535, 2393, 9234, 7443, 
3986, 6506, 1292, 9397, 1621, 9016, 6207, 3661, 
2518, 6102, 8661, 9291, 4709, 2100, 7934, 3020, 
384, 698, 9840, 6818, 1426, 2204, 2529, 1416, 
9847, 9022, 6072, 5382, 1415, 1658, 9178, 1753, 
8164, 470, 1151, 6137, 5839, 3710, 9798, 8357, 
9813, 4811, 7649, 874};
	cout << maxMin(25, arr);

	cout << "---------------------------------------" <<endl<<endl;
	arr = {100, 200, 300, 350, 400, 401, 402};
	k = 3;
	cout << maxMin(k, arr);

	return 0;
}
