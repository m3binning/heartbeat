/* An example Prometheus client. */

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>

#include "microhttpd.h"
#include "prom.h"
#include "promhttp.h"
#include "prom_counter.h"
#include "prom_collector_registry.h"

#include "heart.h"
#include "fuel.h"

struct MHD_Daemon *daemon;
void handle_signal(int signal) {
    printf("\nShutting down...\n");
    prom_collector_registry_destroy(PROM_COLLECTOR_REGISTRY_DEFAULT);
    MHD_stop_daemon(daemon);
    exit(0);
}

int main(int argc, char* argv[]) {
    /* Initialize the default registry. */
    prom_collector_registry_default_init();
    /* Register file-based metrics. */
    heart.init();
    fuel.init();
    /* Set the active registry for the HTTP handler. */
    promhttp_set_active_collector_registry(NULL);
    daemon = promhttp_start_daemon(MHD_USE_SELECT_INTERNALLY, 8000, NULL, NULL);
    if (daemon == NULL) { return 1; }
    signal(SIGINT, handle_signal);
    for(;;) {
        heart.beat();
        fuel.burn();
        sleep(1);
    }
    handle_signal(0);
}

