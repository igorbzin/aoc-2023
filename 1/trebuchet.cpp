#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

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

        for (int i = 0; line_buffer[i] != '\0'; i++) {
            if (isdigit(line_buffer[i])) {
                if (firstNumber == -1) {
                    firstNumber = line_buffer[i] - '0';
                }
                lastNumber = line_buffer[i] - '0';
            }

            if (line_buffer[i] == '\n') {
                break;
            }
        }

        if (firstNumber != -1) {
            counter += firstNumber * 10 + lastNumber;
        }
    }

    printf("The final value is : %d\n", counter);
    fclose(fp);
    return 0;
}
