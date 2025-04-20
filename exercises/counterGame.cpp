#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);
vector<string> split(const string &);

string counterGame(long m) {
    bool winner = 0;
    int index = 63;
		unsigned long n = m;
    for (; index >= 0; index--) {
			if (n == 1) break;
			unsigned long powerOfTwoNumber = 1ULL << index;
//			cout << "Testing " << powerOfTwoNumber << " over " << n << endl;
			if (powerOfTwoNumber == n) {
				if (index & 1 == 1) {
					winner = !winner;
				}
				return winner ? "Louise" : "Richard";
			}
			if ((unsigned long)powerOfTwoNumber > (unsigned long)n) {
					continue;
			}
//			cout << "Removing " << (1ULL << (index)) << " from " << n;
			n -= (1ULL << index);
//			cout << " equals to " << n << endl;
//			index++;
			winner = !winner;
    }

    return winner ? "Louise" : "Richard";
}

int main() {
	vector<long> inputs = {1017175374172081, 629784923222424,
												959765159837254, 1001263837889179,
												587843805747644, 262144,
												17592186044416, 6};
//	inputs = {6};
	for (int i = 0; i < inputs.size(); i++) {
		cout << inputs[i] << " -> " << counterGame(inputs[i]) << endl;
	}
	return 0;
}
