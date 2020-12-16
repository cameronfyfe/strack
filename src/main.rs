#[macro_use]
extern crate prettytable;
pub mod config;
pub mod fn_node;
pub mod analyze;
pub mod report;
pub mod stack_usage;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::{self, process::exit};

use config::{Config, Context};
use serde_json;

fn _main(args: Vec<String>) -> i32 {
    let strack_path = Path::new(&args[0]);
    let strack_function = args[1].as_str();
    let strack_args = args[2..].iter().map(AsRef::as_ref).collect::<Vec<&str>>();

    // Read config file, prepare running context
    let ctx = match Context::new(strack_path) {
        Ok(ctx) => ctx,
        Err(msg) => panic!("Issue reading strack config."),
    };

    // Check if strack is enabled
    if ctx.config.enabled == false {
        return 1;
    }

    // Run Strack Function
    match strack_function {
        "analyze" => {
            analyze::analyze(&ctx, strack_args);
        }
        "report" => {
            report::report(&ctx, strack_args);
        }
        _ => {
            println!("Invalid strack function.");
            return 1;
        }
    }
    // Ok
    0
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
        let args = [
            ".",
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
            "example/Debug/Middlewares/FreeRTOS/portable/port.o",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        let ret = _main(args);

        // Ensure main returns 0 status code
        assert_eq!(ret, 0);

        // Results file exists
        let report_file = Path::new("out/strack_report.json");
        assert_eq!(report_file.exists(), true);

        // Read results file
        let mut buffer = String::new();
        fs::File::open(&report_file)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        let report_file = serde_json::from_str::<report::Report>(&buffer).unwrap();

        // Verify some hardcoded results from report for this example build
        assert_eq!(report_file.total_function_nodes, 811);
        assert_eq!(report_file.num_functions_known_local_stack, 809);
        assert_eq!(report_file.num_functions_known_max_stack, 810);
        assert_eq!(report_file.tracked_functions[0].name, "main");
        assert_eq!(report_file.tracked_functions[0].su_max, 316);
        assert_eq!(report_file.tracked_functions[1].name, "LED_Thread1");
        assert_eq!(report_file.tracked_functions[1].su_max, 124);
        assert_eq!(report_file.tracked_functions[2].name, "LED_Thread2");
        assert_eq!(report_file.tracked_functions[2].su_max, 124);
        assert_eq!(
            report_file.unknown_local_su,
            vec!["Reset_Handler", "ADC3_IRQHandler"]
        );
        assert_eq!(report_file.unknown_max_su, vec!["Reset_Handler"]);
        assert_eq!(
            report_file.missing_children,
            vec!["__libc_init_array", "__aeabi_uldivmod", "memcpy", "memset"]
        );
    }
}
