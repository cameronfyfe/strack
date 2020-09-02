import sys
import os
import subprocess
import json
from terminaltables import AsciiTable
from src.python.utils import *
from src.python.debug_log import *
from src.python.function_node import *
from src.python.su_info import *
from src.python.cg_info import *
from src.python.cg_su_info import *





def strack_die():
    debug_log("***** strack_die() *****")
    exit(1)

# Config passed in from json config file
class StrackConfig:
    
    def __init__(self, config_json_text):
        config_json = json.loads(config_json_text)

        self.enabled = config_json.get("enabled", True)
        self.frame_cost = config_json.get("frame_cost", 0)
        self.allow_function_ptrs = config_json.get("allow_function_ptrs", True)
        self.allow_recursion = config_json.get("allow_recursion", True)
        self.tracked_functions = config_json.get("tracked_functions", [])


def strack_analyze(args):
    # get config from config file
    f_config_json = open(strack_path + "/in/strack_config.json", "r")
    config = StrackConfig(f_config_json.read())
    f_config_json.close()

    if config.enabled is False:
        return

    # gcc object files to run anylsis on
    obj_files = args

    # Create stack usage file
    start_time = time.time()
    create_su_info_file_from_obj_files(su_info_filename, obj_files)
    print("Compiled stack usage in " + str(round(time.time()-start_time, 3)) + " seconds.")

    # Create call graph file
    start_time = time.time()
    create_cg_info_file_from_obj_files(cg_info_filename, obj_files)
    print("Created call graph in " + str(round(time.time()-start_time, 3)) + " seconds.")

    # Analyze
    start_time = time.time()
    analyze_cg_and_su(node_info_filename, report_filename, su_info_filename, cg_info_filename, config)
    print("Analyzed in " + str(round(time.time()-start_time, 3)) + " seconds.")

def strack_report():
    # get config from config file
    f_config_json = open(strack_path + "/in/strack_config.json", "r")
    config = StrackConfig(f_config_json.read())
    f_config_json.close()

    if config.enabled is False:
        return

    report_filename = strack_path + "/out/strack_report.json"

    f_report = open(report_filename, "r")
    report = json.loads(f_report.read())
    f_report.close()

    num_fn_nodes = report["total_function_nodes"] 
    num_fn_with_known_local_stack = report["num_functions_known_local_stack"]
    num_fn_with_known_max_stack = report["num_functions_known_max_stack"]

    pct_fn_with_known_local_stack = get_percent(num_fn_with_known_local_stack, num_fn_nodes)
    pct_fn_with_known_max_stack = get_percent(num_fn_with_known_max_stack, num_fn_nodes)

    print(
        AsciiTable([
            ["Strack Report"],
            ["Total function nodes", num_fn_nodes],
            ["Functions with known local stack usage", num_fn_with_known_local_stack, pct_fn_with_known_local_stack],
            ["Functions with known max stack usage", num_fn_with_known_max_stack, pct_fn_with_known_max_stack]
        ]).table
    )

if __name__ == "__main__":
    
    strack_path = sys.argv[1]

    debug_log_init(strack_path + "/local/strack_log.txt")

    strack_function = sys.argv[2]
    args = sys.argv[3:]
    # args = "example/Debug/Application/SW4STM32/startup_stm32h743xx.o example/Debug/Application/User/main.o example/Debug/Application/User/stm32h7xx_hal_timebase_tim.o example/Debug/Application/User/stm32h7xx_it.o example/Debug/Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.o example/Debug/Drivers/CMSIS/system_stm32h7xx.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_adc.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_adc_ex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_cortex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_dma.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_dma_ex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_exti.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_gpio.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_i2c.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_i2c_ex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_pwr.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_pwr_ex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_rcc.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_rcc_ex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_tim.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_tim_ex.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_uart.o example/Debug/Drivers/STM32H7xx_HAL_Driver/stm32h7xx_hal_uart_ex.o example/Debug/Middlewares/FreeRTOS/CMSIS-RTOS/cmsis_os.o example/Debug/Middlewares/FreeRTOS/croutine.o example/Debug/Middlewares/FreeRTOS/event_groups.o example/Debug/Middlewares/FreeRTOS/list.o example/Debug/Middlewares/FreeRTOS/queue.o example/Debug/Middlewares/FreeRTOS/tasks.o example/Debug/Middlewares/FreeRTOS/timers.o example/Debug/Middlewares/FreeRTOS/portable/heap_4.o example/Debug/Middlewares/FreeRTOS/portable/port.o".split()
    # strack_function = "analyze"
    # args = "analyze Debug/ecatappl.o Debug/ecatslv.o Debug/tasks.o Debug/event_groups.o Debug/list.o Debug/croutine.o Debug/queue.o Debug/timers.o Debug/cmsis_os.o Debug/port.o Debug/heap_3.o Debug/ADC.o Debug/lan9252hw.o Debug/lan9252spi.o Debug/Encoder.o Debug/PWM.o Debug/quad_encoders.o Debug/quad_encoder_configs.o Debug/spi_encoder.o Debug/spi_encoder_interface.o Debug/ams_as5045b_spi.o Debug/ssi_encoder.o Debug/ssi_encoder_interface.o Debug/rls_orbis_ssi.o Debug/ams_as5045b_ssi.o Debug/rls_orbis_payload.o Debug/ams_as5045b_payload.o Debug/EEPROM_interface.o Debug/NORFlash.o Debug/stm32h7xx_hal.o Debug/stm32h7xx_hal_rcc_ex.o Debug/stm32h7xx_hal_flash_ex.o Debug/stm32h7xx_hal_usart.o Debug/stm32h7xx_hal_i2c_ex.o Debug/stm32h7xx_hal_pwr_ex.o Debug/stm32h7xx_hal_dac_ex.o Debug/stm32h7xx_ll_usb.o Debug/stm32h7xx_hal_rcc.o Debug/stm32h7xx_hal_dma_ex.o Debug/stm32h7xx_hal_spi.o Debug/stm32h7xx_hal_spi_ex.o Debug/stm32h7xx_hal_uart.o Debug/stm32h7xx_hal_i2c.o Debug/stm32h7xx_hal_qspi.o Debug/stm32h7xx_hal_pwr.o Debug/stm32h7xx_hal_dac.o Debug/stm32h7xx_hal_dma.o Debug/stm32h7xx_hal_mdma.o Debug/stm32h7xx_hal_adc_ex.o Debug/stm32h7xx_hal_tim_ex.o Debug/stm32h7xx_hal_cortex.o Debug/stm32h7xx_hal_hsem.o Debug/stm32h7xx_hal_uart_ex.o Debug/stm32h7xx_hal_gpio.o Debug/stm32h7xx_hal_flash.o Debug/stm32h7xx_hal_adc.o Debug/stm32h7xx_hal_pcd_ex.o Debug/stm32h7xx_hal_tim.o Debug/stm32h7xx_hal_pcd.o Debug/quadspi.o Debug/usart.o Debug/stm32h7xx_it.o Debug/dac.o Debug/system_stm32h7xx.o Debug/main.o Debug/usb_otg.o Debug/EthercatCore.o Debug/gpio.o Debug/stm32h7xx_hal_timebase_TIM.o Debug/stm32h7xx_hal_msp.o Debug/crc.o Debug/freertos.o Debug/pid_regulator.o Debug/mc_math.o Debug/Filter.o Debug/AngleControl.o Debug/DriveSettings.o Debug/MotorControl.o Debug/FOC.o Debug/MotorControlDefaultSettings.o Debug/SystemControl.o Debug/MotorPresets.o Debug/EtherCatTask.o Debug/startup_stm32h743xx.o".split()
    # strack_function = "report"
    # args = "strack/out/strack_report.json".split()

    su_info_filename = strack_path + "/local/strack_su.json"
    cg_info_filename = strack_path + "/local/strack_cg.json"
    node_info_filename = strack_path + "/out/strack_fn_nodes.json"
    report_filename = strack_path + "/out/strack_report.json"

    if strack_function == "analyze":
        strack_analyze(args)

    elif strack_function == "report":
        strack_report()
