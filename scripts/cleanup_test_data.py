#!/usr/bin/env python3
# cleanup_test_data.py
# Delete test deployment records (id starting with 'test-deploy-')

import sqlite3
import sys

def main():
    db_path = sys.argv[1] if len(sys.argv) > 1 else "db/deployments.db"

    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()
    except Exception as e:
        print(f"Error: Cannot connect to database at {db_path}")
        print(f"Details: {e}")
        sys.exit(1)

    cursor.execute("SELECT COUNT(*) FROM deployments WHERE id LIKE 'test-deploy-%'")
    count = cursor.fetchone()[0]

    if count == 0:
        print("No test records found.")
        conn.close()
        sys.exit(0)

    print(f"Found {count} test records. Deleting...")

    # First delete logs for test deployments
    cursor.execute("DELETE FROM deployment_logs WHERE deployment_id LIKE 'test-deploy-%'")

    # Then delete the deployments
    cursor.execute("DELETE FROM deployments WHERE id LIKE 'test-deploy-%'")

    conn.commit()
    conn.close()

    print(f"Done! Deleted {count} test records and their logs.")

if __name__ == "__main__":
    main()
