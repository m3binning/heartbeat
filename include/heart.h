#ifndef HEART_H
#define HEART_H

typedef struct {
    void (* const init)(void);
    void (* const beat)(void);
} namespace_heart;

extern namespace_heart const heart;

#endif
