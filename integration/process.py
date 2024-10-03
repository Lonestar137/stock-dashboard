import time
import psutil
from prometheus_client import CollectorRegistry, Gauge, push_to_gateway

# Generate and push some periodical data to Graphana.

# Create a registry to hold the metrics
registry = CollectorRegistry()

# Define the Gauge metric for CPU usage
cpu_usage_gauge = Gauge('cpu_usage_percent',
                        'CPU usage percentage',
                        registry=registry)


def push_cpu_usage():
    # Get the current CPU usage as a percentage (across all cores)
    cpu_usage = psutil.cpu_percent(interval=1)

    # Set the value of the Gauge
    cpu_usage_gauge.set(cpu_usage)

    # Push the CPU usage metric to the Pushgateway
    push_to_gateway('http://localhost:9091',
                    job='cpu_usage',
                    registry=registry)


if __name__ == "__main__":
    # Continuously push CPU usage data to the Pushgateway every 10 seconds
    while True:
        push_cpu_usage()
        time.sleep(10)  # Adjust the interval as needed
