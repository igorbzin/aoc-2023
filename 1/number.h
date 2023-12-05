#ifndef NUMBER_H
#define NUMBER_H

typedef enum {
  UNKNOWN,
  ZERO,
  ONE,
  TWO,
  THREE,
  FOUR,
  FIVE,
  SIX,
  SEVEN,
  EIGHT,
  NINE
} Number;

const char *number_as_string(Number n);
const int number_as_digit(Number n);
Number mapStringToNumber(char *str);

#endif // !NUMBER_H
