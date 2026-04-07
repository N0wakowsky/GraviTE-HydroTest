/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : main.h
  * @brief          : Header for main.c file.
  *                   This file contains the common defines of the application.
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2026 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __MAIN_H
#define __MAIN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "stm32f4xx_hal.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

/* Exported types ------------------------------------------------------------*/
/* USER CODE BEGIN ET */

/* USER CODE END ET */

/* Exported constants --------------------------------------------------------*/
/* USER CODE BEGIN EC */

/* USER CODE END EC */

/* Exported macro ------------------------------------------------------------*/
/* USER CODE BEGIN EM */

/* USER CODE END EM */

/* Exported functions prototypes ---------------------------------------------*/
void Error_Handler(void);

/* USER CODE BEGIN EFP */

/* USER CODE END EFP */

/* Private defines -----------------------------------------------------------*/
#define ValveGate4_Pin GPIO_PIN_2
#define ValveGate4_GPIO_Port GPIOE
#define ValveGate3_Pin GPIO_PIN_3
#define ValveGate3_GPIO_Port GPIOE
#define HEATING_I2C_SDA_Pin GPIO_PIN_0
#define HEATING_I2C_SDA_GPIO_Port GPIOF
#define HEATING_I2C_SCL_Pin GPIO_PIN_1
#define HEATING_I2C_SCL_GPIO_Port GPIOF
#define VALVE_FAULT1_Pin GPIO_PIN_2
#define VALVE_FAULT1_GPIO_Port GPIOF
#define ValveGate1_Pin GPIO_PIN_3
#define ValveGate1_GPIO_Port GPIOF
#define ValveGate2_Pin GPIO_PIN_4
#define ValveGate2_GPIO_Port GPIOF
#define VALVE_PWR_EN1_Pin GPIO_PIN_5
#define VALVE_PWR_EN1_GPIO_Port GPIOF
#define FAULT_12V_Pin GPIO_PIN_6
#define FAULT_12V_GPIO_Port GPIOF
#define FAULT_12V_EXTI_IRQn EXTI9_5_IRQn
#define FAULT_5V_Pin GPIO_PIN_7
#define FAULT_5V_GPIO_Port GPIOF
#define PGOOD_12V_Pin GPIO_PIN_8
#define PGOOD_12V_GPIO_Port GPIOF
#define PGOOD_12V_EXTI_IRQn EXTI9_5_IRQn
#define PGOOD_5V_Pin GPIO_PIN_9
#define PGOOD_5V_GPIO_Port GPIOF
#define ADC_5V_Pin GPIO_PIN_2
#define ADC_5V_GPIO_Port GPIOC
#define ADC_12V_Pin GPIO_PIN_3
#define ADC_12V_GPIO_Port GPIOC
#define HEATING_PWM_Pin GPIO_PIN_6
#define HEATING_PWM_GPIO_Port GPIOA
#define BOOT1_Pin GPIO_PIN_2
#define BOOT1_GPIO_Port GPIOB
#define EN_PUMP3_Pin GPIO_PIN_14
#define EN_PUMP3_GPIO_Port GPIOF
#define PUMP_FAULT3_Pin GPIO_PIN_15
#define PUMP_FAULT3_GPIO_Port GPIOF
#define EN_PUMP2_Pin GPIO_PIN_0
#define EN_PUMP2_GPIO_Port GPIOG
#define PUMP_FAULT2_Pin GPIO_PIN_1
#define PUMP_FAULT2_GPIO_Port GPIOG
#define EN_PUMP1_Pin GPIO_PIN_7
#define EN_PUMP1_GPIO_Port GPIOE
#define PUMP_FAULT1_Pin GPIO_PIN_8
#define PUMP_FAULT1_GPIO_Port GPIOE
#define ETH_SPI_RST_Pin GPIO_PIN_9
#define ETH_SPI_RST_GPIO_Port GPIOE
#define ETH_SPI_INT_Pin GPIO_PIN_10
#define ETH_SPI_INT_GPIO_Port GPIOE
#define ETH_SPI_CLK_Pin GPIO_PIN_12
#define ETH_SPI_CLK_GPIO_Port GPIOE
#define ETH_SPI_MISO_Pin GPIO_PIN_13
#define ETH_SPI_MISO_GPIO_Port GPIOE
#define ETH_SPI_MOSI_Pin GPIO_PIN_14
#define ETH_SPI_MOSI_GPIO_Port GPIOE
#define ETH_SPI_CS_Pin GPIO_PIN_15
#define ETH_SPI_CS_GPIO_Port GPIOE
#define ValveGate7_Pin GPIO_PIN_14
#define ValveGate7_GPIO_Port GPIOB
#define ValveGate8_Pin GPIO_PIN_15
#define ValveGate8_GPIO_Port GPIOB
#define PUMP_FAULT4_Pin GPIO_PIN_8
#define PUMP_FAULT4_GPIO_Port GPIOD
#define EN_PUMP4_Pin GPIO_PIN_9
#define EN_PUMP4_GPIO_Port GPIOD
#define PUMP_FAULT5_Pin GPIO_PIN_10
#define PUMP_FAULT5_GPIO_Port GPIOD
#define EN_PUMP5_Pin GPIO_PIN_11
#define EN_PUMP5_GPIO_Port GPIOD
#define PUMP_FAULT6_Pin GPIO_PIN_12
#define PUMP_FAULT6_GPIO_Port GPIOD
#define EN_PUMP6_Pin GPIO_PIN_13
#define EN_PUMP6_GPIO_Port GPIOD
#define PUMP_FAULT7_Pin GPIO_PIN_2
#define PUMP_FAULT7_GPIO_Port GPIOG
#define EN_PUMP7_Pin GPIO_PIN_3
#define EN_PUMP7_GPIO_Port GPIOG
#define PUMP_FAULT8_Pin GPIO_PIN_4
#define PUMP_FAULT8_GPIO_Port GPIOG
#define EN_PUMP8_Pin GPIO_PIN_5
#define EN_PUMP8_GPIO_Port GPIOG
#define EN_PUMP_PWR_Pin GPIO_PIN_6
#define EN_PUMP_PWR_GPIO_Port GPIOG
#define PUMP_PWR_GOOD_Pin GPIO_PIN_7
#define PUMP_PWR_GOOD_GPIO_Port GPIOG
#define VALVE_I2C_SDA_Pin GPIO_PIN_9
#define VALVE_I2C_SDA_GPIO_Port GPIOC
#define VALVE_I2C_SCL_Pin GPIO_PIN_8
#define VALVE_I2C_SCL_GPIO_Port GPIOA
#define VALVE_PWR_EN2_Pin GPIO_PIN_9
#define VALVE_PWR_EN2_GPIO_Port GPIOA
#define VALVE_FAULT2_Pin GPIO_PIN_10
#define VALVE_FAULT2_GPIO_Port GPIOA
#define ValveGate10_Pin GPIO_PIN_11
#define ValveGate10_GPIO_Port GPIOA
#define ValveGate9_Pin GPIO_PIN_12
#define ValveGate9_GPIO_Port GPIOA
#define mem_sck_Pin GPIO_PIN_10
#define mem_sck_GPIO_Port GPIOC
#define mem_miso_Pin GPIO_PIN_11
#define mem_miso_GPIO_Port GPIOC
#define mem_mosi_Pin GPIO_PIN_12
#define mem_mosi_GPIO_Port GPIOC
#define mem__wp_Pin GPIO_PIN_0
#define mem__wp_GPIO_Port GPIOD
#define mem__ce_Pin GPIO_PIN_1
#define mem__ce_GPIO_Port GPIOD
#define mem__hold_Pin GPIO_PIN_2
#define mem__hold_GPIO_Port GPIOD
#define PIEZO_PWR_EN_Pin GPIO_PIN_4
#define PIEZO_PWR_EN_GPIO_Port GPIOD
#define ValveGate12_Pin GPIO_PIN_9
#define ValveGate12_GPIO_Port GPIOG
#define ValveGate11_Pin GPIO_PIN_10
#define ValveGate11_GPIO_Port GPIOG
#define ValveGate6_Pin GPIO_PIN_11
#define ValveGate6_GPIO_Port GPIOG
#define ValveGate5_Pin GPIO_PIN_12
#define ValveGate5_GPIO_Port GPIOG
#define USER_GPIO1_Pin GPIO_PIN_5
#define USER_GPIO1_GPIO_Port GPIOB
#define ST_LINK_TX_Pin GPIO_PIN_6
#define ST_LINK_TX_GPIO_Port GPIOB
#define ST_LINK_RX_Pin GPIO_PIN_7
#define ST_LINK_RX_GPIO_Port GPIOB
#define PIEZODRV_I2C_SCL_Pin GPIO_PIN_8
#define PIEZODRV_I2C_SCL_GPIO_Port GPIOB
#define PIEZODRV_I2C_SDA_Pin GPIO_PIN_9
#define PIEZODRV_I2C_SDA_GPIO_Port GPIOB
#define USER_LED1_Pin GPIO_PIN_0
#define USER_LED1_GPIO_Port GPIOE
#define USER_LED2_Pin GPIO_PIN_1
#define USER_LED2_GPIO_Port GPIOE

/* USER CODE BEGIN Private defines */

/* USER CODE END Private defines */

#ifdef __cplusplus
}
#endif

#endif /* __MAIN_H */
