

int g[5];

cout << g[1] << endl;

Array g(5);

cout << g[1] << endl;


elementType operator[](int index) {
    return this->arr[index];
}
