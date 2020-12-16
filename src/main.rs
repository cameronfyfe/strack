use std::env;
use std::process::Command;

fn _main(args_vec: Vec<String>) -> i32 {

    // Call strack.py with same command line args
    let status = Command::new("python3")
                         .arg("strack.py")
                         .args(args_vec)
                         .status()
                         .expect("process failed to execute");

    match status.code() {
        Some(code) => code,
        None       => 0
    }
}

fn main() {
    _main(env::args().skip(1).collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_main() {
        let args = [".",
                    "analyze",
                    "example/Debug/Application/SW4STM32/startup_stm32h743xx.o",
                    "example/Debug/Application/User/main.o",
                    "example/Debug/Application/User/stm32h7xx_hal_timebase_tim.o",
                    "example/Debug/Application/User/stm32h7xx_it.o",
                    "example/Debug/Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.o",
                    "example/Debug/Drivers/CMSIS/system_stm32h7xx.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_adc.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_adc_ex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_cortex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_dma.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_dma_ex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_exti.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_gpio.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_i2c.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_i2c_ex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_pwr.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_pwr_ex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_rcc.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_rcc_ex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_tim.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_tim_ex.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_uart.o",
                    "example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_uart_ex.o",
                    "example/Debug/Middlewares/FreeRTOS/CMSIS-RTOS/cmsis_os.o",
                    "example/Debug/Middlewares/FreeRTOS/croutine.o",
                    "example/Debug/Middlewares/FreeRTOS/event_groups.o",
                    "example/Debug/Middlewares/FreeRTOS/list.o",
                    "example/Debug/Middlewares/FreeRTOS/queue.o",
                    "example/Debug/Middlewares/FreeRTOS/tasks.o",
                    "example/Debug/Middlewares/FreeRTOS/timers.o",
                    "example/Debug/Middlewares/FreeRTOS/portable/heap_4.o",
                    "example/Debug/Middlewares/FreeRTOS/portable/port.o"]
                    .iter().map(|&s| s.into()).collect();

        let ret = _main(args);

        // Ensure main returns 0 status code
        assert_eq!(ret, 0);
        // Results file exists     
        let result_file_exists = std::path::Path::new("out/strack_report.json").exists(); 
        assert_eq!(result_file_exists, true);
    }
}
