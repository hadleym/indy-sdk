#!/usr/bin/env python3
from vcx.api import vcx_init

import asyncio


async def main():
    await vcx_init.vcx_init('ENABLE_TEST_MODE')
   

if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main)
