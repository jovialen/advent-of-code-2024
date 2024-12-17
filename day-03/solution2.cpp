#include <iostream>
#include <fstream>
#include <iterator>
#include <string>
#include <filesystem>
#include <regex>
#include <optional>

std::optional<std::string> read_input(const std::filesystem::path &path) {
  if (!std::filesystem::exists(path)) {
    std::cerr << "File " << path << " does not exist" << std::endl;
    return std::nullopt;
  }

  std::ifstream ifs(path);
  return std::string(
    std::istreambuf_iterator<char>(ifs),
    std::istreambuf_iterator<char>()
  );
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cout << "Usage: ./solution [input]" << std::endl;
    return 0;
  }

  std::optional<std::string> parsed = read_input(argv[1]);
  if (!parsed.has_value()) {
    return 1;
  }
  auto input = *parsed;

  int sum = 0;
  bool on = true;

  std::regex pattern("mul\\(\\d+,\\d+\\)|do\\(\\)|don't\\(\\)");
  for (std::smatch match; std::regex_search(input, match, pattern);) {
    auto term = match.str();
    input = match.suffix();

    if (term == "do()") {
      on = true;
      continue;
    } else if (term == "don't()") {
      on = false;
      continue;
    }

    if (on) {
      int a, b;
      std::sscanf(term.c_str(), "mul(%d,%d)", &a, &b);

      sum += a * b;
    }
  }

  std::cout << sum << std::endl;

  return 0;
}
