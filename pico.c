#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/adc.h"

#define ADC_PIN 26
#define ADC_NUM 0

int main() {
	stdio_init_all();

	adc_init();
	adc_gpio_init(ADC_PIN);
	adc_select_input(ADC_NUM);

	while (true) {
		uint adc_raw = adc_read();
		printf("%d\n", adc_raw);
		sleep_ms(1);
		// sleep_us(125);
	}
}
