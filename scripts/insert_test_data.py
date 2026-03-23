#!/usr/bin/env python3
# insert_test_data.py
# Insert 1000 test deployment records into the database

import sqlite3
import sys
import uuid
from datetime import datetime, timedelta

def main():
    db_path = sys.argv[1] if len(sys.argv) > 1 else "db/deployments.db"

    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()
    except Exception as e:
        print(f"Error: Cannot connect to database at {db_path}")
        print(f"Details: {e}")
        sys.exit(1)

    statuses = ["pending", "running", "success", "failed", "cancelled"]
    branches = ["main", "develop", "feature/test", "hotfix/bug", "release/v1.0"]

    print(f"Inserting 1000 test deployment records into {db_path}...")

    base_time = datetime.now()

    for i in range(1, 1001):
        deployment_id = f"test-deploy-{i}"
        project_num = (i - 1) // 20 + 1
        project_name = f"test-project-{project_num}"
        repo_url = f"https://github.com/test/{project_name}.git"
        branch = branches[i % 5]
        project_type = "php"
        status = statuses[i % 5]
        install_command = "composer install"
        build_command = "php artisan migrate"
        docker_service = "php"
        working_dir = "/app"

        # created_at: spread across last 30 days
        days_ago = (1000 - i) * 3 / 100
        hours_ago = (1000 - i) % 24 * 4
        created_at = base_time - timedelta(days=days_ago, hours=hours_ago)

        started_at = None
        finished_at = None
        if status != "pending":
            started_at = created_at
            if status != "running":
                finished_at = created_at + timedelta(hours=1)

        cursor.execute("""
            INSERT INTO deployments (id, project_name, repo_url, branch, project_type, status,
                install_command, build_command, extra_command, docker_service, working_dir,
                created_at, started_at, finished_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            deployment_id, project_name, repo_url, branch, project_type, status,
            install_command, build_command, "", docker_service, working_dir,
            created_at.isoformat(),
            started_at.isoformat() if started_at else None,
            finished_at.isoformat() if finished_at else None
        ))

        if i % 100 == 0:
            print(f"Inserted {i} / 1000 records...")
            conn.commit()

    conn.commit()
    conn.close()
    print("Done! Inserted 1000 test records.")
    print("Test records have id starting with 'test-deploy-'")

if __name__ == "__main__":
    main()
