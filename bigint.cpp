/*
 * Honor Statement:
 * I pledge that this program represents my own work.
 * I have not received or given unauthorized assistance.
 */

#include <algorithm>
#include <cctype>
#include <climits> /* For INT_MAX */
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

using std::cout;    // for console output
using std::endl;    // for new line
using std::flush;   // for flushing output buffer
using std::isdigit; // for checking if char is digit
using std::max;     // for finding max of two numbers easy to write
using std::ostream; // for overloading insertion operator
using std::setw;    // for formatted width output
using std::string;  // for string data type hard to write
using std::system;  // for system commands like "date"
using std::vector;  // for internal digit storage

class BigInt {
private:
  vector<char> v; // Digits 0-9, stored in reverse order (v[0] is ones place)

  // Helper to convert BigInt to int (for recursion limits)
  int toInt() const {
    int res = 0;
    int mult = 1;
    for (size_t i = 0; i < v.size(); ++i) {
      if (v[i] < 0 || v[i] > 9) {
        continue;
      }
      res += (int)v[i] * mult;
      mult *= 10;
    }
    return res;
  }

  // Tail recursive helpers
  BigInt fiboTail(int n, const BigInt &a, const BigInt &b) {
    if (n == 0)
      return a;
    return fiboTail(n - 1, b, a + b);
  }

  BigInt factTail(int n, const BigInt &acc) {
    if (n <= 1)
      return acc;
    return factTail(n - 1, acc * BigInt(n));
  }

public:
  // Constructors
  BigInt() { v.push_back(0); }

  BigInt(int n) {
    if (n == 0) {
      v.push_back(0);
    } else {
      while (n > 0) {
        v.push_back(n % 10);
        n /= 10;
      }
    }
  }

  BigInt(string s) {
    if (s.empty()) {
      v.push_back(0);
    } else {
      // Loop from end to start
      for (int i = (int)s.length() - 1; i >= 0; --i) {
        if (isdigit(s[i])) {
          v.push_back(s[i] - '0');
        }
      }
      // Remove leading zeros if any
      while (v.size() > 1 && v.back() == 0) {
        v.pop_back();
      }
      if (v.empty()) {
        v.push_back(0);
      }
    }
  }

  // Accessor
  int size() const { return (int)v.size(); }

  // Operators
  BigInt operator+(const BigInt &other) const {
    BigInt res;
    res.v.clear();
    int carry = 0;
    size_t n = max(v.size(), other.v.size());
    for (size_t i = 0; i < n || carry; ++i) {
      int sum = carry + (i < v.size() ? v[i] : 0) +
                (i < other.v.size() ? other.v[i] : 0);
      res.v.push_back(sum % 10);
      carry = sum / 10;
    }
    return res;
  }

  BigInt operator-(const BigInt &other) const {
    // Assume *this >= other
    BigInt res;
    res.v.clear();
    int borrow = 0;
    for (size_t i = 0; i < v.size(); ++i) {
      int sub = v[i] - borrow - (i < other.v.size() ? other.v[i] : 0);
      if (sub < 0) {
        sub += 10;
        borrow = 1;
      } else {
        borrow = 0;
      }
      res.v.push_back(sub);
    }
    // Remove leading zeros
    while (res.v.size() > 1 && res.v.back() == 0) {
      res.v.pop_back();
    }
    return res;
  }

  // Needed for subtraction with int
  BigInt operator-(int n) const { return *this - BigInt(n); }

  BigInt operator*(const BigInt &other) const {
    if ((v.size() == 1 && v[0] == 0) ||
        (other.v.size() == 1 && other.v[0] == 0)) {
      return BigInt(0);
    }
    BigInt res;
    // Result size is at most sum of sizes
    res.v.assign(v.size() + other.v.size(), 0);

    for (size_t i = 0; i < v.size(); ++i) {
      long long carry = 0;
      for (size_t j = 0; j < other.v.size() || carry; ++j) {
        long long cur =
            res.v[i + j] +
            (long long)v[i] * (j < other.v.size() ? other.v[j] : 0) + carry;
        res.v[i + j] = cur % 10;
        carry = cur / 10;
      }
    }
    // Remove leading zeros
    while (res.v.size() > 1 && res.v.back() == 0) {
      res.v.pop_back();
    }
    return res;
  }

  // Division by repeated subtraction
  BigInt operator/(const BigInt &other) const {
    if (other == BigInt(0)) {
      return BigInt(0);
    }
    BigInt count(0);
    BigInt temp = *this;
    while (temp >= other) {
      temp = temp - other;
      count = count + BigInt(1);
    }
    return count;
  }

  BigInt operator%(const BigInt &other) const {
    if (other == BigInt(0))
      return BigInt(0);
    BigInt temp = *this;
    while (temp >= other) {
      temp = temp - other;
    }
    return temp;
  }

  // Comparison operators
  bool operator==(const BigInt &other) const { return v == other.v; }

  bool operator<(const BigInt &other) const {
    if (v.size() != other.v.size())
      return v.size() < other.v.size();
    for (int i = (int)v.size() - 1; i >= 0; --i) {
      if (v[i] != other.v[i])
        return v[i] < other.v[i];
    }
    return false; // Equal
  }

  bool operator>=(const BigInt &other) const { return !(*this < other); }

  // Pre-increment
  BigInt operator++() {
    *this = *this + BigInt(1);
    return *this;
  }

  // Post-increment
  BigInt operator++(int) {
    BigInt temp = *this;
    ++(*this);
    return temp;
  }

  // Index operator
  BigInt operator[](int index) {
    if (index >= 0 && index < (int)v.size()) {
      return BigInt((int)v[index]);
    }
    return BigInt(0);
  }

  void print() {
    for (auto it = v.rbegin(); it != v.rend(); ++it) {
      cout << (int)*it;
    }
  }

  BigInt fibo() { return fiboTail(toInt(), BigInt(0), BigInt(1)); }

  BigInt fact() { return factTail(toInt(), BigInt(1)); }

  BigInt collatz(bool printSteps) {
    BigInt curr = *this;
    BigInt steps(0);

    while (true) {
      if (printSteps) {
        cout << curr;
        if (!(curr == BigInt(1))) {
          cout << ". ";
        }
      }

      if (curr == BigInt(1)) {
        break;
      }

      if (curr.v[0] % 2 == 0) {
        // even
        int remainder = 0;
        for (int i = (int)curr.v.size() - 1; i >= 0; --i) {
          int val = curr.v[i] + remainder * 10;
          curr.v[i] = val / 2;
          remainder = val % 2;
        }
        while (curr.v.size() > 1 && curr.v.back() == 0) {
          curr.v.pop_back();
        }
      } else {
        // odd
        curr = curr * BigInt(3) + BigInt(1);
      }

      steps = steps + BigInt(1);
    }
    cout << endl << ":->";
    return steps;
  }

  friend BigInt operator+(int n, BigInt b) { return BigInt(n) + b; }

  friend ostream &operator<<(ostream &out, const BigInt &n) {
    bool hasWidth = out.width() > 0;
    out.width(0);
    if (n.v.size() > 12) {
      // Scientific notation: 1.234567e13
      out << (int)n.v.back() << ".";
      for (int i = 1; i <= 6; ++i) {
        if ((int)n.v.size() - 1 - i >= 0) {
          out << (int)n.v[n.v.size() - 1 - i];
        } else {
          out << "0";
        }
      }
      out << "e" << (n.v.size() - 1);
    } else {
      if (hasWidth)
        out << " ";
      for (auto it = n.v.rbegin(); it != n.v.rend(); ++it) {
        out << (int)*it;
      }
    }
    return out;
  }
};

// Main Program - DO NOT MODIFY
int main() {
  int space = 10;
  cout << "\a\nTestUnit:\n" << flush; // std:: flush clears the buffer
  cout << "User Name:" << flush;
  system("whoami");
  system("date");
  BigInt n1(25);
  BigInt s1("25");
  BigInt n2(1234);
  BigInt s2("1234");
  BigInt n3(n2);
  BigInt X(3000);
  BigInt Y(50);
  BigInt Z1(123);
  BigInt Z2("989345275647");
  BigInt Z3(X.fibo());
  BigInt imax = INT_MAX;
  BigInt big("9223372036854775807");
  cout << "n1(int) :" << setw(space) << n1 << endl;
  cout << "s1(str) :" << setw(space) << s1 << endl;
  cout << "n2(int) :" << setw(space) << n2 << endl;
  cout << "s2(str) :" << setw(space) << s2 << endl;
  cout << "n3(n2) :" << setw(space) << n3 << endl;
  cout << "X.fibo(1234):" << setw(space) << X.fibo() << endl;
  cout << "Y.fact(50) :" << setw(space) << Y.fact() << endl;
  cout << "imax :" << setw(space) << imax << endl;
  cout << "big :" << setw(space) << big << endl;
  cout << "big.print(): ";
  big.print();
  cout << endl;
  cout << n2 << "/" << n1 << " = " << n2 / n1 << " rem " << n2 % n1 << endl;
  cout << "fibo(" << X << ") = " << X.fibo() << endl;
  cout << "fact(" << Y << ") = " << Y.fact() << endl;
  bool printSteps = true;
  cout << "steps for collatz(" << Z1 << "):" << Z1.collatz(printSteps) << endl;
  printSteps = false;
  // 1348 steps per Wikipedia https://en.wikipedia.org/wiki/Collatz_conjecture
  cout << "steps for collatz(" << Z2 << "):" << Z2.collatz(printSteps) << endl;
  cout << "steps for collatz(" << Z3 << "):" << Z3.collatz(printSteps) << endl;
  cout << "10 + n1 = " << BigInt(10) + n1 << endl;
  cout << "n1 + 10 = " << n1 + BigInt(10) << endl;
  cout << "(n1 == s1)? --> " << ((n1 == s1) ? "true" : "false") << endl;
  cout << "n1++ = ? --> before:" << n1++ << " after:" << n1 << endl;
  cout << "++s1 = ? --> before:" << ++s1 << " after:" << s1 << endl;
  cout << "s2 * big = ? --> " << s2 * big << endl;
  cout << "big * s2 = ? --> " << big * s2 << endl;
  cout << endl;
  system("date");
  return 0;
}
