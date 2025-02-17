# IoTHub

## Опис
IoTHub - це потужна платформа для управління IoT-пристроями, яка включає модулі для реєстрації, управління, моніторингу та аналізу даних з пристроїв.

## Структура проекту
Проект розділений на кілька шарів для покращення читабельності та підтримуваності коду:

- **Domain**: Основна бізнес-логіка та правила.
- **Application**: Інтерфейси, юзкейси та реалізації для роботи з даними.
- **Infrastructure**: Реалізація деталей інфраструктури, таких як моделі даних, контролери та утиліти.
- **Presentation**: Візуалізація даних та взаємодія з користувачем.

## Встановлення
1. Клонуйте репозиторій:
    ```bash
    git clone <URL репозитарію>
    ```
2. Перейдіть до каталогу проекту:
    ```bash
    cd iot_hub
    ```
3. Встановіть необхідні залежності:
    ```bash
    cargo build
    ```

## Запуск
Для запуску проекту виконайте наступну команду:
```bash
cargo run
```

## Структура каталогів
```plaintext
iot_hub/
├── src/
│   ├── domain/
│   │   ├── entities/
│   │   │   ├── Device.rs
│   │   │   └── SensorData.rs
│   │   ├── repositories/
│   │   │   └── DeviceRepository.rs
│   │   ├── services/
│   │   │   └── DeviceService.rs
│   │   └── use_cases/
│   │       └── ManageDevice.rs
│   ├── infrastructure/
│   │   ├── models/
│   │   │   └── DeviceModel.rs
│   │   ├── repositories/
│   │   │   └── DeviceRepositoryImpl.rs
│   │   ├── controllers/
│   │   │   └── DeviceController.rs
│   ├── application/
│       └── main.rs
├── config/
│   └── config.rs
├── Cargo.toml
└── README.md
```

## Опис компонентів
### Domain
- **Device.rs**: Клас сутності пристрою.
- **SensorData.rs**: Клас сутності даних сенсора.
- **DeviceRepository.rs**: Інтерфейс репозиторію пристроїв.

### Application
- **ManageDevice.rs**: Юзкейс для управління пристроями.
- **DeviceService.rs**: Сервіс для управління пристроями.

### Infrastructure
- **DeviceModel.rs**: Модель даних пристрою.
- **DeviceRepositoryImpl.rs**: Реалізація репозиторію пристроїв.
- **DeviceController.rs**: Контролер для управління пристроями.

## Ліцензія
Цей проект ліцензовано під ліцензією MIT. Для отримання додаткової інформації див. файл LICENSE.
