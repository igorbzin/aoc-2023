#include "number.h"
#include <cstdio>
#include <cstring>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  FILE *fp = fopen("input.txt", "r");
  if (fp == NULL) {
    perror("Error opening file");
    return 1;
  }

  int counter = 0;
  char line_buffer[128];

  while (fgets(line_buffer, sizeof(line_buffer), fp)) {
    int firstNumber = -1;
    int lastNumber = -1;

    short examine_from = 0;

    for (int i = 0; line_buffer[i] != '\0'; i++) {
      if (isdigit(line_buffer[i])) {
        if (firstNumber == -1) {
          firstNumber = line_buffer[i] - '0';
        }
        lastNumber = line_buffer[i] - '0';
      } else {
        char written_digit[i];
        memcpy(written_digit, line_buffer + examine_from,
               (i + 1) * sizeof(char));
        printf("EXAMINING %s\n", written_digit);
        Number string_as_number = mapStringToNumber(written_digit);

        if (string_as_number != UNKNOWN) {
          int readNumber = number_as_digit(string_as_number);
          printf("FOUND NUMBER %d\n", readNumber);

          if (firstNumber == -1) {
            firstNumber = readNumber;
          }

          lastNumber = readNumber;
          examine_from = i - 1;
        }
      }

      if (line_buffer[i] == '\n') {
        break;
      }
    }

    printf("Adding numbers %d for line %s\n", firstNumber * 10 + lastNumber,
           line_buffer);

    if (firstNumber != -1) {
      counter += firstNumber * 10 + lastNumber;
    }
  }

  printf("The final value is : %d\n", counter);
  fclose(fp);
  return 0;
}
