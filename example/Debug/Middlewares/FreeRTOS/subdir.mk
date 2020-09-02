################################################################################
# Automatically-generated file. Do not edit!
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/croutine.c \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/event_groups.c \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/list.c \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/queue.c \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/tasks.c \
/home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/timers.c 

OBJS += \
./Middlewares/FreeRTOS/croutine.o \
./Middlewares/FreeRTOS/event_groups.o \
./Middlewares/FreeRTOS/list.o \
./Middlewares/FreeRTOS/queue.o \
./Middlewares/FreeRTOS/tasks.o \
./Middlewares/FreeRTOS/timers.o 

C_DEPS += \
./Middlewares/FreeRTOS/croutine.d \
./Middlewares/FreeRTOS/event_groups.d \
./Middlewares/FreeRTOS/list.d \
./Middlewares/FreeRTOS/queue.d \
./Middlewares/FreeRTOS/tasks.d \
./Middlewares/FreeRTOS/timers.d 


# Each subdirectory must supply rules for building sources it contributes
Middlewares/FreeRTOS/croutine.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/croutine.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Middlewares/FreeRTOS/croutine.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"
Middlewares/FreeRTOS/event_groups.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/event_groups.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Middlewares/FreeRTOS/event_groups.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"
Middlewares/FreeRTOS/list.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/list.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Middlewares/FreeRTOS/list.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"
Middlewares/FreeRTOS/queue.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/queue.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Middlewares/FreeRTOS/queue.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"
Middlewares/FreeRTOS/tasks.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/tasks.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Middlewares/FreeRTOS/tasks.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"
Middlewares/FreeRTOS/timers.o: /home/cameron/STM32CubeIDE/workspace_1.4.0/FreeRTOS_ThreadCreation/Middlewares/Third_Party/FreeRTOS/Source/timers.c
	arm-none-eabi-gcc "$<" -mcpu=cortex-m7 -std=gnu11 -g3 -DUSE_HAL_DRIVER -DSTM32H743xx -DUSE_STM32H7XX_NUCLEO_144_MB1364 -c -I../../../Inc -I../../../Drivers/CMSIS/Device/ST/STM32H7xx/Include -I../../../Drivers/STM32H7xx_HAL_Driver/Inc -I../../../Drivers/BSP/STM32H7xx_Nucleo -I../../../Drivers/BSP/Components/Common -I../../../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F -I../../../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS -I../../../Middlewares/Third_Party/FreeRTOS/Source/include -I../../../Utilities/Log -I../../../Utilities/Fonts -I../../../Utilities/CPU -I../../../Drivers/CMSIS/Include -Os -ffunction-sections -Wall -fstack-usage -MMD -MP -MF"Middlewares/FreeRTOS/timers.d" -MT"$@" --specs=nano.specs -mfpu=fpv5-d16 -mfloat-abi=hard -mthumb -o "$@"

