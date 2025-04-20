#include <bits/stdc++.h>

using namespace std;

void plusMinus(vector<int> arr) {
    float positives = 0;
    float negatives = 0;
    float zeros = 0;
    
    if (arr.size() == 0) {
        cout << "0.000000" << endl << "0.000000" << endl << "0.000000" << endl;
        return;
    }
    
    // Loop through the vector with the iterator    
    for (auto it = arr.begin(); it != arr.end(); ++it) {
        if (*it > 0) 
            positives++;
        else if (*it < 0)
            negatives++;
        else
            zeros ++;
    }
		float positivesRatio = positives / arr.size();
		cout << positives << " " << negatives << " " << zeros<< endl;	
		cout << setprecision(6) << positivesRatio	<< endl <<
				(negatives / arr.size()) << endl <<
				(zeros / arr.size());

}

int main() {
	vector<int> arr = {-4, 3, -9, 0, 4, 1};
	plusMinus(arr);
	return 0;
}
