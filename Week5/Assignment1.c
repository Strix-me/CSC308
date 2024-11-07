#include <stdio.h>

int main() {
    int charac;

    printf("Choose an operation to perform:\n");
    printf("1. Character\n");
    printf("2. Float\n");
    printf("3. Integer\n");

    printf("Enter your choice (1-3): ");
    scanf("%d", &charac);

    switch (charac) {
        case 1: {
            char ch;
            printf("Enter your Character: ");
           
            scanf(" %c", &ch);

            printf("Next four characters:\n");
            for (int i = 1; i <= 4; i++) {
                printf("%c (ASCII: %d)\n", ch + i, ch + i);
            }
            printf("\nSize of character: %lu bytes\n", sizeof(ch));
            break;
        }
        case 2: {
            float f;
            printf("Enter your Float: ");
           
            scanf("%f", &f);

            printf("\nNext four Floats:\n");
            for (int i = 1; i <= 4; i++) {
                printf("%.2f\n", f + i * 3);
            }
            printf("Size of float: %lu bytes\n", sizeof(f));
            break;
        }

        case 3: {
            int num;
            printf("Enter your Integer:");
           
            scanf("%d", &num);

            printf("\nNext four Integers:\n");
            for (int i = 1; i <= 4; i++) {
                printf("%d\n", num + i * 3);
            }
            printf("Size of Integers: %lu bytes\n", sizeof(num));
            break;
        }
        
         default:
            printf("Invalid choice.\n");
            break;
    }

    return 0;
}
