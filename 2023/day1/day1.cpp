#include <iostream>
using namespace std;

#include <fstream>
#include <iterator>
#include <regex>

ifstream input("input.txt");

int main() {
  regex nonNumeric("[^0-9]");
  int result = 0;

  for (string line; getline(input, line);) {
    string numericLine = regex_replace(line, nonNumeric, "");
    result += stoi(string(1, numericLine[0]) + string(1, numericLine.back()));
  }

  cout << result;
  return 0;
}
