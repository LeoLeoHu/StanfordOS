#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);
volatile unsigned GPIO16OUT = 0x00040000;
volatile unsigned GPIO16SET = 0x00010000;
volatile unsigned GPIO16CLR = 0x00010000;
volatile unsigned GPIO05OUT = 0x00008000;
volatile unsigned GPIO05SET = 0x00000020;
volatile unsigned GPIO05CLR = 0x00000020;

static void spin_sleep_us(unsigned int us) {
	for (unsigned int i = 0; i < us * 6; i++) {
		asm volatile("nop");
	}
}

static void spin_sleep_ms(unsigned int ms) {
	spin_sleep_us(ms * 1000);
}

int main(void) {
	// STEP 1: Set GPIO Pin 16 as output.
	*GPIO_FSEL1 |= GPIO16OUT; 

	// STEP 2: Continuously set and clear GPIO 16.
	while (1) {
		*GPIO_SET0 |= GPIO16SET;
		spin_sleep_ms(500);
		*GPIO_CLR0 |= GPIO16CLR;
		spin_sleep_ms(500);
	}
}
