// Named enum
typedef enum
{
    STATUS_OK,
    STATUS_ERROR
} Status;

// Unnamed enum
enum
{
    LEVEL_LOW,
    LEVEL_MEDIUM,
    LEVEL_HIGH
};

// Named struct
typedef struct
{
    int x;
    int y;
} Coordinate;

// Unnamed struct
struct
{
    float width;
    float height;
} rectangle;

// Named union
typedef union
{
    int intValue;
    double doubleValue;
    char charValue;
} Data;

// Unnamed union
union
{
    short s;
    long l;
} number;