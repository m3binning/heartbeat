#include <stddef.h>
#include <stdio.h>

#include "prom_counter.h"
#include "prom_collector_registry.h"
#include "heart.h"

static prom_counter_t* heartbeats;

static void init(void) {
    heartbeats = prom_collector_registry_must_register_metric(prom_counter_new("heartbeats", "heart-beats since start", 0, NULL));
}

static void beat(void) {
    printf(".");
    fflush(stdout);
    prom_counter_inc(heartbeats, NULL);
}

namespace_heart const heart = { init, beat };
