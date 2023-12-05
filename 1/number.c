#include "number.h"
#include <stdio.h>
#include <string.h>

const char *number_strings[11] = {"unknown", "zero",  "one",  "two",
                                  "three",   "four",  "five", "six",
                                  "seven",   "eight", "nine"};

const int number_digits[11] = {-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

const char *number_as_string(Number n) {
  printf("Number as string %d\n", n);
  return number_strings[n];
}

const int number_as_digit(Number n) { return number_digits[n]; }

Number mapStringToNumber(char *str) {
  if (strstr(str, "zero") != NULL) {
    return ZERO;
  } else if (strstr(str, "one") != NULL) {
    return ONE;
  } else if (strstr(str, "two") != NULL) {
    return TWO;
  } else if (strstr(str, "three") != NULL) {
    return THREE;
  } else if (strstr(str, "four") != NULL) {
    return FOUR;
  } else if (strstr(str, "five") != NULL) {
    return FIVE;
  } else if (strstr(str, "six") != NULL) {
    return SIX;
  } else if (strstr(str, "seven") != NULL) {
    return SEVEN;
  } else if (strstr(str, "eight") != NULL) {
    return EIGHT;
  } else if (strstr(str, "nine") != NULL) {
    return NINE;
  }

  return UNKNOWN;
}
