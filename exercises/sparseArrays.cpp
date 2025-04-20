#include <bits/stdc++.h>
#include <regex>
using namespace std;

vector<int> matchingStrings(vector<string> strings, vector<string> queries) {
    vector<int> results(queries.size());

    for (int i = 0; i < queries.size(); i++) {
        for (int j = 0; j < strings.size(); j++) {
            regex pattern(queries[i]);
            if (regex_match(strings[j], pattern)) {
                results[i]++;
            }
        }
    }

    return results;
}

int main() {
	vector<string> queries = {"aba", "xzxb", "ab"};
	vector<string> strings = {"aba", "baba", "aba", "xzxb"};
	vector<int> res = matchingStrings(strings, queries);
	return 0;
}
