#include <stdio.h>

int main() {
    char foodCode;
    int quantity;
    int price = 0;
    int totalCost = 0;
    char continueOrder;

    printf("Welcome to Mama Cas Restaurant!\n");
    printf("Menu:\n");
    printf("P = Poundo Yam/Edinkaiko Soup - N3,200\n");
    printf("F = Fried Rice & Chicken - N3,000\n");
    printf("A = Amala & Ewedu Soup - N2,500\n");
    printf("E = Eba & Egusi Soup - N2,000\n");
    printf("W = White Rice & Stew - N2,500\n");

    do {
        printf("Enter the food code (P/F/A/E/W): ");
        scanf(" %c", &foodCode);
        printf("Enter quantity: ");
        scanf("%d", &quantity);

        switch (foodCode) {
            case 'P':
            case 'p':
                price = 3200;
                break;
            case 'F':
            case 'f':
                price = 3000;
                break;
            case 'A':
            case 'a':
                price = 2500;
                break;
            case 'E':
            case 'e':
                price = 2000;
                break;
            case 'W':
            case 'w':
                price = 2500;
                break;
            default:
                printf("Invalid food code! Please try again.\n");
                continue;
        }

        totalCost += price * quantity;
        printf("Do you want to order another item? (Y/N): ");
        scanf(" %c", &continueOrder);

    } while (continueOrder == 'Y' || continueOrder == 'y');

    printf("Total cost for your order: N%d\n", totalCost);

    return 0;
}