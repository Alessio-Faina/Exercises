#include <bits/stdc++.h>
#include <string>
#include <iostream>

using namespace std;

int main()
{
    string result = "";
    int n = 157;
		int divider = 10;
    while (n > 0) {
        int residue = n % divider;
        n -= residue;
				n /= 10;
        string temp = "";
        temp.push_back(residue + '0');
        result.insert(0, temp);
    }
    cout <<  "-----> " << result;
    return 0;
}
