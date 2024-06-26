## A command tool get system information
- [ ] [User information]
```shell
+-------+--------------+-----+-----+-------+
| INDEX |     USER     | UID | GID | LOGIN |
+-------+--------------+-----+-----+-------+
| 0     | root         | 0   | 0   |       |
+-------+--------------+-----+-----+-------+
| 1     | _mbsetupuser | 248 | 248 |       |
+-------+--------------+-----+-----+-------+
```
- [ ] [process information]
```shell
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| PID  |  STATUS  |                           NAME                           | DISK-USAGE | MEMORY-USAGE | CMD |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| 1304 | Unknown  | dprivacyd                                                | 0          | 0            |     |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| 4759 | Sleeping | DiskUnmountWatcher                                       | 24576      | 3391488      |     |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| 1095 | Sleeping | fudHelperAgent                                           | 122880     | 5783552      |     |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| 949  | Unknown  | UARPUpdaterServiceAFU                                    | 0          | 0            |     |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| 730  | Runnable | askpermissiond                                           | 19275776   | 19382272     |     |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
| 773  | Runnable | networkserviceproxy                                      | 2576384    | 32964608     |     |
+------+----------+----------------------------------------------------------+------------+--------------+-----+
```
- [ ] [Host information]
```shell
+---------------+-------------+---------------+--------------+---------+
|   host-name   | system-name |    kernel     | architecture | up-time |
+---------------+-------------+---------------+--------------+---------+
| MacBook.local | Darwin      | MacOS 14.2.1  | arm64        | 6273    |
+---------------+-------------+---------------+--------------+---------+
```
- [ ] [Disk information]
```shell
+-------+--------------+------+------------+----------------------+----------------+-----------+
| Index |     Disk     | Type | FileSystem |       Mounted        | Capability/GiB | Removable |
+-------+--------------+------+------------+----------------------+----------------+-----------+
| 0     | Macintosh HD | SSD  | apfs       | /                    | 480.79         | false     |
+-------+--------------+------+------------+----------------------+----------------+-----------+
| 1     | Macintosh HD | SSD  | apfs       | /System/Volumes/Data | 480.79         | false     |
+-------+--------------+------+------------+----------------------+----------------+-----------+
```
- [ ] [Memory information]
```shell
+------------------+----------------------+-----------------+-----------------+----------------+---------------+---------------+
| Total-Memory/GiB | Available-Memory/GiB | Used-Memory/GiB | Free-Memory/GiB | Total-Swap/GiB | Used-Swap/GiB | Free-Swap/GiB |
| 32.00            | 18.48                | 17.12           | 6.43            | 0.00           | 0.00          | 0.00          |
+------------------+----------------------+-----------------+-----------------+----------------+---------------+---------------+
```
