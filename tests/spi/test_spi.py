import mpsse
import time

spi = mpsse.MPSSE()
spi.Open(0x18d1, 0x0304, mpsse.SPI0, 1000000, mpsse.LSB, mpsse.IFACE_B)
spi.PinHigh(1)
time.sleep(1)
spi.Start()
buf = spi.Read(256)
spi.Stop()

print(":".join("{:02x}".format(ord(c)) for c in buf))
