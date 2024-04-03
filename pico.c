#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/adc.h"

#define ADC_PIN 26
#define ADC_NUM 0

bool timer_callback(struct repeating_timer *t) {
	char c[2];

	uint adc_raw = adc_read();
	c[0] = adc_raw >> 8;
	c[1] = adc_raw;
	write(1, c, 2);

	return true;
}

int main() {
	stdio_init_all();
	stdio_set_translate_crlf(&stdio_usb, false);

	adc_init();
	adc_gpio_init(ADC_PIN);
	adc_select_input(ADC_NUM);

	struct repeating_timer timer;
	add_repeating_timer_us(25, timer_callback, NULL, &timer);

	while (1) {
		tight_loop_contents();
	}
}
