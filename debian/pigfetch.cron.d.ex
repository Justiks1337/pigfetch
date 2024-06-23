#
# Regular cron jobs for the pigfetch package.
#
0 4	* * *	root	[ -x /usr/bin/pigfetch_maintenance ] && /usr/bin/pigfetch_maintenance
