#include <bits/stdc++.h>
using namespace std;

vector<int> arr;
vector<unsigned long long> sum;

unsigned long long generate_combi(int idx) {
    if (sum[idx] == 0) {
        unsigned long long combi = 0;
        int next = idx + 1;

        while (next < arr.size() && arr[next] - arr[idx] <= 3) {
            combi += generate_combi(next);
            next++;
        }

        sum[idx] = combi;
    }

    return sum[idx];
}

int main() {
    int num;

    arr.push_back(0);

    while (scanf("%d", &num) == 1) {
        arr.push_back(num);
    }

    sort(arr.begin(), arr.end());
    sum.resize(arr.size(), 0);

    sum[arr.size() - 1] = 1;

    generate_combi(0);

    printf("%lld\n", sum[0]);

    return 0;
}