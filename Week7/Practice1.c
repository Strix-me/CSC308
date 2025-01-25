#include <stdio.h>

int main(){
    int a;
    float b;
    double c;
    char name[6]="BASITO";
    int *p2a = &a;
    a = 10;

    //Pointer Arithmtic
    for(int i = 0; i < 7; i++)
    {
        printf("%c\n", name[i]);
    }
    puts("");
    char *p2name = name;
     



    /*a = 10;
    printf("%i\n", *p2a);
    a = 90;
    printf("%i\n", *p2a);
    *p2a = 5;
    printf("%i\n", *p2a);*/



    //scanf("%s", name);
    //printf("%s\n", name);

    /*int *p2a;
    p2a = &a;

    printf("%d\n", a);
    printf("%p\n", p2a);*/
}