################################################################################
# Automatically-generated file. Do not edit!
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.c 

OBJS += \
./Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.o 

C_DEPS += \
./Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.d 


# Each subdirectory must supply rules for building sources it contributes
Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Drivers/BSP/STM32H7xx_Nucleo/stm32h7xx_nucleo.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"

