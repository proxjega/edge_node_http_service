#!/usr/bin/env python3

import json
import random
import statistics
import subprocess
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone


REQUEST_COUNT = 100


def enable_netem() -> None:
     cmd = [
          "sudo",
          "tc",
          "qdisc",
          "replace",
          "dev",
          "lo",
          "root",
          "netem",
          "delay",
          "100ms",
          "20ms",
          "distribution",
          "normal",
     ]
     subprocess.run(cmd, check=True)
     print("netem enabled on lo: delay=100ms jitter=20ms")


def disable_netem() -> None:
     cmd = ["sudo", "tc", "qdisc", "del", "dev", "lo", "root"]
     result = subprocess.run(cmd, check=False)
     if result.returncode == 0:
          print("netem disabled on lo")
     else:
          print("netem cleanup skipped (no qdisc on lo)")


def build_payload() -> dict[str, object]:
     return {
          "sensor_id": "sensor_001",
          "value": round(random.uniform(-30.0, 50.0), 2),
          "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
     }


def send_request(payload: dict[str, object], url) -> tuple[int, float]:
     body = json.dumps(payload).encode("utf-8")
     request = urllib.request.Request(
          url,
          data=body,
          headers={"Content-Type": "application/json"},
          method="POST",
     )

     start = time.perf_counter()
     
     try:
          with urllib.request.urlopen(request, timeout=10) as response:
               response.read()
               status = response.status
     except urllib.error.HTTPError as error:
          error.read()
          status = error.code

     elapsed = time.perf_counter() - start
     
     return status, elapsed


def main() -> None:
    # local
    url = "http://localhost:8080/data"
    durations_local = []
    statuses_local = {}

    print(f"Sending {REQUEST_COUNT} requests to {url}")

    for index in range(REQUEST_COUNT):
        status, duration = send_request(build_payload(), url)
        durations_local.append(duration)
        statuses_local[status] = statuses_local.get(status, 0) + 1
        print(f"{index + 1:03d}: status={status} time={duration * 1000:.2f} ms")

    overall_duration_local = sum(durations_local)

    # remote with netem enabled
    url = "http://localhost:8081/data"
    durations_remote = []
    statuses_remote = {}

    try:
        enable_netem()
        print(f"Sending {REQUEST_COUNT} requests to {url}")

        for index in range(REQUEST_COUNT):
            status, duration = send_request(build_payload(), url)
            durations_remote.append(duration)
            statuses_remote[status] = statuses_remote.get(status, 0) + 1
            print(f"{index + 1:03d}: status={status} time={duration * 1000:.2f} ms")
    finally:
        disable_netem()

    overall_duration_remote = sum(durations_remote)


    print()
    print("Local summary")
    print(f"  total_requests: {REQUEST_COUNT}")
    print(f"  total_time: {overall_duration_local:.3f} s")
    print(f"  avg_time: {statistics.mean(durations_local) * 1000:.2f} ms")
    print(f"  min_time: {min(durations_local) * 1000:.2f} ms")
    print(f"  max_time: {max(durations_local) * 1000:.2f} ms")
    print(f"  p95_time: {statistics.quantiles(durations_local, n=20)[18] * 1000:.2f} ms")
    print(f"  status_counts: {statuses_local}")

    print()
    print("Remote summary")
    print(f"  total_requests: {REQUEST_COUNT}")
    print(f"  total_time: {overall_duration_remote:.3f} s")
    print(f"  avg_time: {statistics.mean(durations_remote) * 1000:.2f} ms")
    print(f"  min_time: {min(durations_remote) * 1000:.2f} ms")
    print(f"  max_time: {max(durations_remote) * 1000:.2f} ms")
    print(f"  p95_time: {statistics.quantiles(durations_remote, n=20)[18] * 1000:.2f} ms")
    print(f"  status_counts: {statuses_remote}")


if __name__ == "__main__":
    main()
     
     
