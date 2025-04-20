#ifndef FUEL_H
#define FUEL_H

typedef struct {
    void (* const init)(void);
    int (* const burn)(void);
} namespace_fuel;

extern namespace_fuel const fuel;

#endif
