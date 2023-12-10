#include <stdio.h>
#include <string.h>

#define MAX_RED 12
#define MAX_GREEN 13
#define MAX_BLUE 14
#define LINE_SEPARATOR ":"
#define SECTION_SEPARATOR ";"
#define COLOR_SECTION_SEPARATOR ","

int checkColorValidity(int colorNumber, const char *colorName) {
    char colorChar = colorName[0];
    int isColorValid = 1;
    if ((colorChar == 'r' && colorNumber > MAX_RED) ||
        (colorChar == 'b' && colorNumber > MAX_BLUE) ||
        (colorChar == 'g' && colorNumber > MAX_GREEN)) {
        isColorValid = 0;
    }
    return isColorValid;
}

int isSectionValid(char *section, int *minRed, int *minBlue, int *minGreen) {
    int isGameSectionValid = 1;
    char *colorSection = strtok(section, COLOR_SECTION_SEPARATOR);

    while (colorSection != NULL) {
        int colorNumber;
        char colorName[8];
        sscanf(colorSection, "%d %s", &colorNumber, colorName);

        if (strcmp(colorName, "red") == 0) {
            if (colorNumber > *minRed) {
                *minRed = colorNumber;
            }
        } else if (strcmp(colorName, "blue") == 0) {
            if (colorNumber > *minBlue) {
                *minBlue = colorNumber;
            }
        } else if (strcmp(colorName, "green") == 0) {
            if (colorNumber > *minGreen) {
                *minGreen = colorNumber;
            }
        }

        isGameSectionValid = checkColorValidity(colorNumber, colorName);
        colorSection = strtok(NULL, COLOR_SECTION_SEPARATOR);
    }
    return isGameSectionValid;
}


int processLine(char *line, int *sumOfPowers) {
    int fewestRed = 1;
    int fewestBlue = 1;
    int fewestGreen = 1;
    int isLineValid = 1;
    char *gamePart = strtok(line, LINE_SEPARATOR);
    int gameNumber;
    sscanf(gamePart, "Game %d", &gameNumber);

    if (gamePart != NULL) {
        char *restOfLine = line + strlen(gamePart) + 1;
        char *gameSection = strtok_r(restOfLine, SECTION_SEPARATOR, &restOfLine);

        while (gameSection != NULL) {
            isLineValid = isSectionValid(gameSection, &fewestRed, &fewestBlue, &fewestGreen);

            gameSection = strtok_r(NULL, SECTION_SEPARATOR, &restOfLine);
        }
    }

    *sumOfPowers += fewestRed * fewestBlue * fewestGreen;
    printf("Fewest red: %d, fewest blue: %d, fewest green: %d \n", fewestRed, fewestBlue, fewestGreen);
    return isLineValid ? gameNumber : 0;
}

int main() {
    FILE *fp = fopen("../input.txt", "r");
    if (fp == NULL) {
        perror("Error opening file");
        return 1;
    }

    char lineBuffer[512];
    int sumOfIds = 0;
    int sumOfPowers = 0;

    while (fgets(lineBuffer, sizeof(lineBuffer), fp)) {
        sumOfIds += processLine(lineBuffer, &sumOfPowers);
    }

    fclose(fp);


    printf("Sum of powers of all games: %d\n", sumOfPowers);
    printf("Sum of IDs of valid games: %d\n", sumOfIds);
    return 0;
}