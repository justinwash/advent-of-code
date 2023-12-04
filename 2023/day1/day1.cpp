#include <iostream>
using namespace std;

#include <fstream>
#include <iterator>
#include <map>

std::string extractNumerals(const std::string& input) {
  std::map<std::string, std::string> numericWords = {
      {"zero", "0"},  {"one", "1"},  {"two", "2"}, {"three", "3"},
      {"four", "4"},  {"five", "5"}, {"six", "6"}, {"seven", "7"},
      {"eight", "8"}, {"nine", "9"}};

  std::string result;

  for (size_t i = 0; i < input.size(); ++i) {
    if (!isalpha(input[i])) {
      result += input[i];
    }

    else {
      std::string currentWord;

      for (size_t j = i; j < input.size(); ++j) {
        currentWord += std::tolower(input[j]);

        auto it = numericWords.find(currentWord);
        if (it != numericWords.end()) {
          result += it->second;
          break;
        }
      }
    }
  }

  return result;
}

ifstream input("input.txt");

int main() {
  int result = 0;

  for (string line; getline(input, line);) {
    string numericLine = extractNumerals(line);
    result += stoi(string(1, numericLine[0]) + string(1, numericLine.back()));
  }

  cout << "Result: \n";
  cout << result;
  return 0;
}
