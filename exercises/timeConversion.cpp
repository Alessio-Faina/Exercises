#include <bits/stdc++.h>
#include <regex>

using namespace std;

string timeConversion(string s) {
    const regex pattern("^[0-2]\\d:{1}[0-5]\\d:{1}[0-5]\\d[AP]M{1}$");
    if (!regex_match(s, pattern)) {
        return "";
    }

    string result = s;
    if (s[8] == 'A' && s[0] == '1' && s[1] == '2') {
        result[0] = '0';
        result[1] = '0';
    }
    else if (s[8] == 'P') {
        if (!(s[0] == '1' && s[1] == '2')) {
            int hour = ((s[0] - 48) * 10) + (s[1] - 48);
            hour += 12;
            string hoursString = std::to_string(hour);
            result[0] = hoursString[0];
            result[1] = hoursString[1];
        }
    }

    result.resize(s.size() - 2);
    return result;
}

int main() {
	string str = "07:00:00PM";
	string result = timeConversion(str);
	cout << result;
	return 0;
}
