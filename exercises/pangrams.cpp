#include <bits/stdc++.h>

using namespace std;

string pangrams(string s) {
    int result = 0;
    for (int i = 0; i < s.length(); i++) {
        if (s[i] == ' ' ) {
            result |= (1 << 25);
        } else if (s[i] <= 'Z') {
            result |= (1 << (s[i] - 'A'));
        } else {
            result |= (1 << (s[i] - 'a'));
        }
    }
    for (int i = 0; i < 26; i++) {
        if (!(result & (1 << i))) {
            return "not pangram";
        }
    }
    return "pangram";
}

int main() {
	cout << pangrams("We promptly judged antique ivory buckles for the next prize");
	cout << pangrams("We promptly judged antique ivory buckles for the prize");
	return 0;
}
