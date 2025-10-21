#!/usr/bin/env python3
"""
Focus Time Data Analysis Script

Analyzes your focus_log.jsonl file to provide insights about your focus patterns.

Usage:
    python3 analyze_focus_data.py

Requirements:
    pip install pandas matplotlib seaborn
"""

import json
import sys
from pathlib import Path
from collections import Counter
from datetime import datetime
import os

# Try to import optional dependencies
try:
    import pandas as pd
    import matplotlib.pyplot as plt
    import seaborn as sns
    HAS_VISUALIZATION = True
except ImportError:
    HAS_VISUALIZATION = False
    print("Note: Install pandas, matplotlib, and seaborn for visualizations:")
    print("  pip install pandas matplotlib seaborn")
    print()


def find_log_file():
    """Find the focus_log.jsonl file in the config directory."""
    if sys.platform == 'darwin':  # macOS
        config_dir = Path.home() / 'Library' / 'Application Support' / 'com.focustime.app'
    elif sys.platform == 'linux':
        config_dir = Path.home() / '.config' / 'focus-time'
    elif sys.platform == 'win32':
        config_dir = Path(os.getenv('APPDATA')) / 'com.focustime.app'
    else:
        print(f"Unsupported platform: {sys.platform}")
        return None

    log_file = config_dir / 'focus_log.jsonl'

    if not log_file.exists():
        print(f"Log file not found at: {log_file}")
        print("Make sure you've completed at least one check-in first.")
        return None

    return log_file


def load_entries(log_file):
    """Load all entries from the JSONL file."""
    entries = []
    with open(log_file, 'r') as f:
        for line in f:
            try:
                entries.append(json.loads(line))
            except json.JSONDecodeError as e:
                print(f"Warning: Skipping invalid JSON line: {e}")
    return entries


def basic_stats(entries):
    """Calculate and display basic statistics."""
    print("=" * 60)
    print("FOCUS TIME - DATA ANALYSIS")
    print("=" * 60)
    print()

    total = len(entries)
    print(f"Total Check-ins: {total}")

    if total == 0:
        print("No check-ins recorded yet!")
        return

    # First and last check-in dates
    first_date = datetime.fromisoformat(entries[0]['timestamp'].replace('Z', '+00:00'))
    last_date = datetime.fromisoformat(entries[-1]['timestamp'].replace('Z', '+00:00'))
    days_tracked = (last_date - first_date).days + 1

    print(f"Date Range: {first_date.date()} to {last_date.date()}")
    print(f"Days Tracked: {days_tracked}")
    print()

    # Status distribution
    print("-" * 60)
    print("STATUS DISTRIBUTION")
    print("-" * 60)

    statuses = [e['reported_status'] for e in entries]
    status_counts = Counter(statuses)

    for status, count in status_counts.most_common():
        percentage = (count / total) * 100
        bar = '█' * int(percentage / 2)  # Scale to 50 chars max
        print(f"{status:25s} {count:4d} ({percentage:5.1f}%) {bar}")

    print()

    # On-task rate
    on_task_count = sum(1 for e in entries if e['reported_status'] == 'On Task')
    on_task_rate = (on_task_count / total) * 100

    print("-" * 60)
    print("FOCUS METRICS")
    print("-" * 60)
    print(f"On-Task Rate: {on_task_rate:.1f}%")

    # Distraction analysis
    distraction_keywords = ['Social', 'Email', 'Chat', 'Other Distraction']
    distractions = [e for e in entries if any(kw in e['reported_status'] for kw in distraction_keywords)]
    distraction_rate = (len(distractions) / total) * 100
    print(f"Distraction Rate: {distraction_rate:.1f}%")

    breaks = sum(1 for e in entries if 'Break' in e['reported_status'])
    break_rate = (breaks / total) * 100
    print(f"Break Rate: {break_rate:.1f}%")

    print()


def time_of_day_analysis(entries):
    """Analyze focus patterns by time of day."""
    print("-" * 60)
    print("TIME OF DAY ANALYSIS")
    print("-" * 60)

    hourly_stats = {}

    for entry in entries:
        dt = datetime.fromisoformat(entry['timestamp'].replace('Z', '+00:00'))
        hour = dt.hour

        if hour not in hourly_stats:
            hourly_stats[hour] = {'total': 0, 'on_task': 0}

        hourly_stats[hour]['total'] += 1
        if entry['reported_status'] == 'On Task':
            hourly_stats[hour]['on_task'] += 1

    print("\nHour | Check-ins | On-Task Rate")
    print("-----|-----------|-------------")

    for hour in sorted(hourly_stats.keys()):
        stats = hourly_stats[hour]
        on_task_pct = (stats['on_task'] / stats['total']) * 100
        print(f"{hour:02d}:00 |    {stats['total']:3d}    | {on_task_pct:5.1f}%")

    print()


def goal_analysis(entries):
    """Analyze focus by goal type."""
    print("-" * 60)
    print("GOAL ANALYSIS")
    print("-" * 60)

    goals = {}

    for entry in entries:
        goal = entry.get('session_goal', '(no goal)')
        if not goal:
            goal = '(no goal)'

        if goal not in goals:
            goals[goal] = {'total': 0, 'on_task': 0}

        goals[goal]['total'] += 1
        if entry['reported_status'] == 'On Task':
            goals[goal]['on_task'] += 1

    print("\nTop Goals by Check-in Count:")
    print()

    sorted_goals = sorted(goals.items(), key=lambda x: x[1]['total'], reverse=True)

    for i, (goal, stats) in enumerate(sorted_goals[:10], 1):
        on_task_pct = (stats['on_task'] / stats['total']) * 100
        print(f"{i}. {goal[:50]}")
        print(f"   Check-ins: {stats['total']}, On-Task: {on_task_pct:.1f}%")
        print()


def notes_with_keywords(entries, keywords):
    """Find entries with specific keywords in notes."""
    print("-" * 60)
    print(f"ENTRIES WITH KEYWORDS: {', '.join(keywords)}")
    print("-" * 60)

    matches = []
    for entry in entries:
        notes = entry.get('notes', '').lower()
        if any(kw.lower() in notes for kw in keywords):
            matches.append(entry)

    if not matches:
        print("No matches found.")
    else:
        for entry in matches[:20]:  # Show first 20
            dt = datetime.fromisoformat(entry['timestamp'].replace('Z', '+00:00'))
            print(f"\n{dt}: {entry['reported_status']}")
            print(f"  Goal: {entry.get('session_goal', 'N/A')}")
            print(f"  Note: {entry.get('notes', 'N/A')}")

    print()


def create_visualizations(entries):
    """Create visualizations if matplotlib is available."""
    if not HAS_VISUALIZATION:
        print("Skipping visualizations (matplotlib not installed)")
        return

    print("-" * 60)
    print("Creating visualizations...")
    print("-" * 60)

    # Convert to DataFrame
    df = pd.DataFrame(entries)
    df['timestamp'] = pd.to_datetime(df['timestamp'])
    df['hour'] = df['timestamp'].dt.hour
    df['date'] = df['timestamp'].dt.date

    # Create figure with subplots
    fig, axes = plt.subplots(2, 2, figsize=(15, 10))
    fig.suptitle('Focus Time Analysis', fontsize=16)

    # 1. Status distribution pie chart
    status_counts = df['reported_status'].value_counts()
    axes[0, 0].pie(status_counts.values, labels=status_counts.index, autopct='%1.1f%%')
    axes[0, 0].set_title('Status Distribution')

    # 2. Check-ins over time
    daily_counts = df.groupby('date').size()
    axes[0, 1].plot(daily_counts.index, daily_counts.values, marker='o')
    axes[0, 1].set_title('Check-ins per Day')
    axes[0, 1].set_xlabel('Date')
    axes[0, 1].set_ylabel('Count')
    axes[0, 1].tick_params(axis='x', rotation=45)

    # 3. Hourly on-task rate
    hourly_ontask = df[df['reported_status'] == 'On Task'].groupby('hour').size()
    hourly_total = df.groupby('hour').size()
    hourly_rate = (hourly_ontask / hourly_total * 100).fillna(0)
    axes[1, 0].bar(hourly_rate.index, hourly_rate.values)
    axes[1, 0].set_title('On-Task Rate by Hour')
    axes[1, 0].set_xlabel('Hour of Day')
    axes[1, 0].set_ylabel('On-Task Rate (%)')
    axes[1, 0].set_ylim([0, 100])

    # 4. Status frequency by day
    status_by_date = df.groupby(['date', 'reported_status']).size().unstack(fill_value=0)
    status_by_date.plot(kind='bar', stacked=True, ax=axes[1, 1])
    axes[1, 1].set_title('Status Breakdown by Day')
    axes[1, 1].set_xlabel('Date')
    axes[1, 1].set_ylabel('Count')
    axes[1, 1].tick_params(axis='x', rotation=45)
    axes[1, 1].legend(loc='upper left', fontsize=8)

    plt.tight_layout()

    # Save to file
    output_file = 'focus_analysis.png'
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    print(f"\nVisualization saved to: {output_file}")

    # Optionally show
    # plt.show()


def main():
    """Main analysis function."""
    log_file = find_log_file()

    if not log_file:
        return

    print(f"Loading data from: {log_file}")
    print()

    entries = load_entries(log_file)

    if not entries:
        print("No entries found in log file.")
        return

    # Run analyses
    basic_stats(entries)
    time_of_day_analysis(entries)
    goal_analysis(entries)

    # Create visualizations
    create_visualizations(entries)

    print("-" * 60)
    print("Analysis complete!")
    print()
    print("Want to search notes? Example:")
    print('  python3 analyze_focus_data.py --notes "stuck" "bug"')
    print()


if __name__ == '__main__':
    # Simple command-line argument handling
    if len(sys.argv) > 1 and sys.argv[1] == '--notes':
        log_file = find_log_file()
        if log_file:
            entries = load_entries(log_file)
            keywords = sys.argv[2:] if len(sys.argv) > 2 else ['progress', 'stuck']
            notes_with_keywords(entries, keywords)
    else:
        main()
