#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/adc.h"

#define ADC_PIN 26
#define ADC_NUM 0

int main() {
	stdio_init_all();
	stdio_set_translate_crlf(&stdio_usb, false);

	adc_init();
	adc_gpio_init(ADC_PIN);
	adc_select_input(ADC_NUM);

	while (true) {
		char c[2];

		uint adc_raw = adc_read();

		// printf("%d\n", adc_raw);

		c[0] = adc_raw >> 8;
		c[1] = adc_raw;
		write(1, c, 2);

		// sleep_ms(1);
		sleep_us(250);
		// sleep_us(125);
	}
}
