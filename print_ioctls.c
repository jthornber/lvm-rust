#include <stdio.h>
#include <linux/ioctl.h>
#include <linux/dm-ioctl.h>

#define P(cmd) \
	printf("%s = %ull,\n", #cmd, cmd)

int main(int argc, char **argv)
{
	P(DM_VERSION);
	P(DM_REMOVE_ALL);
	P(DM_LIST_DEVICES);
	P(DM_DEV_CREATE);
	P(DM_DEV_REMOVE);
	P(DM_DEV_RENAME);
	P(DM_DEV_SUSPEND);
	P(DM_DEV_STATUS);
	P(DM_DEV_WAIT);
	P(DM_TABLE_LOAD);
	P(DM_TABLE_CLEAR);
	P(DM_TABLE_DEPS);
	P(DM_TABLE_STATUS);
	P(DM_LIST_VERSIONS);
	P(DM_TARGET_MSG);
	P(DM_DEV_SET_GEOMETRY);

	return 0;
}
