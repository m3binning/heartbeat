#include <stddef.h>
#include <stdio.h>

#include "prom.h"
#include "prom_gauge.h"
#include "prom_collector_registry.h"
#include "fuel.h"

#define FUEL_MAX 3600
#define FUEL_DELTA_1HZ 2

/* Maintain internal state. */
static unsigned int fuel_remaining = FUEL_MAX;
/* Reflect state in a gauge. */
prom_gauge_t* fuel_gauge;

static void init(void) {
    fuel_gauge = prom_collector_registry_must_register_metric(
            prom_gauge_new("fuel_gauge", "gauge of fuel of some resource", 0, NULL)
    );
    if(prom_gauge_set(fuel_gauge, 3600, NULL) > 0) { abort(); }
}

static void burn(void) {
    if(fuel_remaining >= FUEL_DELTA_1HZ) {
        fuel_remaining -= FUEL_DELTA_1HZ;
        /* This could be set to the internal fuel state, or it could be equally subtract as below. */
        prom_gauge_sub(fuel_gauge, FUEL_DELTA_1HZ, NULL);
        printf("**");
        /* Purely for display purposes, if the amount of fuel burned is a
         * multiple of 40, go to the next line. */
        if(0 == (FUEL_MAX - fuel_remaining) % 40) { printf("\n"); }
        fflush(stdout);
    }
}

namespace_fuel const fuel = { init, burn };
