import sys
import time


def progress_bar():
    for i in range(101):
        progress = "=" * (i // 2)
        spaces = " " * (50 - (i // 2))
        sys.stdout.write(f"\r[{progress}{spaces}] {i}%")
        sys.stdout.flush()
        time.sleep(0.1)
    print()  # 完成后换行


# 运行
progress_bar()
