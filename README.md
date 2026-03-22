# Chat Hexagonal - Sistema de Chat con Arquitectura Hexagonal

Aplicación Rust que implementa un sistema de chat pequeño siguiendo la **arquitectura hexagonal (puertos y adaptadores)**. 

## 🎯 Descripción

Este proyecto demuestra cómo la arquitectura hexagonal facilita la creación de sistemas escalables y desacoplados. El chat permite:
- **Enviar mensajes** desde múltiples usuarios
- **Recuperar mensajes** almacenados
- **Persistencia** en archivos JSON
- **Modo cliente y servidor** separados

## 🏗️ Arquitectura Hexagonal

### Estructura del Proyecto

```
src/
├── main.rs                             (Punto de entrada)
├── lib.rs                              (Biblioteca exportada)
│
├── domain/                             (Núcleo - Lógica de negocio pura)
│   ├── models/
│   │   ├── message.rs                 (Entidad - Mensaje)
│   │   ├── user.rs                    (Entidad - Usuario)
│   │   └── conversation.rs            (Entidad - Conversación)
│   ├── ports/
│   │   ├── message_storage_port.rs    (Puerto - Almacenamiento de mensajes)
│   │   └── server_connection_port.rs  (Puerto - Conexión servidor-cliente)
│   └── services/
│       └── chat_service.rs            (Servicio - Lógica de chat)
│
├── application/                        (Casos de uso)
│   └── use_cases/
│       ├── send_message_use_case.rs   (UC - Enviar mensaje)
│       └── fetch_messages_use_case.rs (UC - Recuperar mensajes)
│
├── adapters/                           (Implementaciones concretas)
│   ├── input/
│   │   ├── client_cli.rs              (Adaptador - Cliente chat CLI)
│   │   └── server_cli.rs              (Adaptador - Servidor chat CLI)
│   └── output/
│       ├── json_storage.rs            (Adaptador - Almacenamiento JSON)
│       └── tcp_connection.rs          (Adaptador - Conexión TCP)
│
└── infrastructure/
    └── bootstrap.rs                   (Configuración e inyección de dependencias)
```

### Capas Hexagonales

#### 1. **Domain (Núcleo)**
La capa más interna. Contiene lógica pura de negocio sin dependencias externas.

**Modelos:**
- `Message`: Representa un mensaje (id, sender, content, timestamp)
- `User`: Representa un usuario (id, username)
- `Conversation`: Colección de mensajes

**Puertos (Interfaces):**
- `MessageStoragePort`: Define cómo guardar/recuperar mensajes (agnóstico)
- `ServerConnectionPort`: Define conexión servidor-cliente

**Servicios:**
- `ChatService`: Orquesta la lógica de envío/recepción
  - `send_message()`: Envía y almacena un mensaje
  - `fetch_all_messages()`: Obtiene todos los mensajes
  - `fetch_last_messages()`: Obtiene últimos N mensajes

---

#### 2. **Application (Casos de Uso)**
Orquestación de la lógica del negocio.

- `SendMessageUseCase`: Ejecuta envío de mensaje → ChatService → Storage
- `FetchMessagesUseCase`: Ejecuta recuperación → ChatService → Storage

---

#### 3. **Adapters (Implementaciones Concretas)**

**Input (Lado izquierdo - Usuarios):**
- `ClientCliAdapter`: Interfaz CLI interactiva para usuario
  - Lee comandos desde stdin
  - Soporta: `/send <username> "message"`, `/fetch [count]`, `/exit`

- `ServerCliAdapter`: Interfaz CLI para servidor
  - Monitorea estado del chat
  - Soporta: `/status`, `/exit`

**Output (Lado derecho - Sistemas Externos):**
- `JsonStorageAdapter`: Implementa `MessageStoragePort`
  - Persiste mensajes en `data/chat_messages.json`
  - Métodos: save_message(), get_all_messages(), get_last_messages()

- `TcpConnectionAdapter`: Implementa `ServerConnectionPort`
  - Gestiona conexiones TCP (escalable para v2)
  - Para v1: client y server leen del mismo archivo JSON

---

#### 4. **Infrastructure (Bootstrap)**
Configuración e inyección de dependencias.

```rust
pub fn run() {
    // Detecta modo desde args: "server" o default "client"
    match mode {
        "server" => run_chat_server(),
        _ => run_chat_client(),  // Default: cliente
    }
}
```

---

## 🚀 Uso

### Prerequisitos
- Rust 1.70+ (`rustup`)
- `cargo`

### Instalación

```bash
git clone <repo>
cd chat
cargo build --release
```

### Ejecutar Cliente

Enviar y recibir mensajes (modo interactivo - por defecto):

```bash
cargo run
# o explícitamente:
cargo run -- client
```

**Comandos disponibles:**
```
/send <username> "message"  - Envía un mensaje
/fetch [count]              - Obtiene últimos N mensajes (default: 10)
/exit                       - Sale del chat
```

**Ejemplo:**
```
> /send alice "Hello everyone!"
Message sent: alice [alice-1774139258]

> /send bob "Hi Alice!"
Message sent: bob [bob-1774139259]

> /fetch
--- Last 2 messages ---
[alice-1774139258] alice: Hello everyone!
[bob-1774139259] bob: Hi Alice!
------------------------
```

---

### Ejecutar Servidor

Monitor y gestión del chat:

```bash
cargo run -- server
```

**Comandos disponibles:**
```
/status  - Muestra estadísticas del servidor (# mensajes, usuarios activos)
/exit    - Apaga el servidor
```

---

## 📊 Arquitectura - Flujo de Datos

### Envío de Mensaje

```
Client CLI Input
    ↓
ClientCliAdapter (/send alice "Hello")
    ↓
SendMessageUseCase::execute()
    ↓
ChatService::send_message() [Validación]
    ↓
Message::new() [Creación entidad]
    ↓
MessageStoragePort::save_message() [Abstracción]
    ↓
JsonStorageAdapter [Implementación concreta]
    ↓
data/chat_messages.json [Persistencia]
```

### Recuperación de Mensajes

```
Client CLI Input
    ↓
ClientCliAdapter (/fetch 10)
    ↓
FetchMessagesUseCase::execute_last()
    ↓
ChatService::fetch_last_messages()
    ↓
MessageStoragePort::get_last_messages() [Abstracción]
    ↓
JsonStorageAdapter [Implementación concreta]
    ↓
data/chat_messages.json [Lectura]
    ↓
Vec<Message> → Imprime en CLI
```

---

## 🧪 Testing

### Ejecutar Tests

```bash
# Tests unitarios (domain + application + adapters)
cargo test --lib

# Tests de integración
cargo test --test integration_test

# Todos los tests
cargo test
```

**Coverage (16 pruebas):**
- ✅ Message creation & validation (3 tests)
- ✅ User creation & validation (2 tests)
- ✅ Conversation management (3 tests)
- ✅ ChatService (3 tests)
- ✅ Use cases (4 tests)
- ✅ JSON Storage (2 tests)

---

## 📂 Almacenamiento

### Estructura JSON

```json
[
  {
    "id": "alice-1774139258",
    "sender": "alice",
    "content": "Hello from Alice!",
    "timestamp": 1774139258
  },
  {
    "id": "bob-1774139259",
    "sender": "bob",
    "content": "Hi Alice!",
    "timestamp": 1774139259
  }
]
```

**Ubicación:** `data/chat_messages.json`

---

## 🔄 Ventajas de la Arquitectura Hexagonal

1. **Independencia de Base de Datos**: El domain no depende de cómo se guarden mensajes
2. **Testeable**: Services y use cases se prueban con mocks sin I/O real
3. **Escalable**: Cambiar de JSON a PostgreSQL solo requiere nuevoAdapter
4. **Flexible**: Cliente y servidor pueden funcionar independientemente
5. **Mantenible**: Lógica de negocio centralizada en domain

---

## 🚧 Mejoras Futuras (v2)

- [ ] Implementar conexión TCP real entre cliente y servidor
- [ ] Agregar autenticación y autorización
- [ ] Soporte para canales/grupos
- [ ] Base de datos (SQLite/PostgreSQL) en lugar de JSON
- [ ] Async/await con Tokio para mejor concurrencia
- [ ] API REST + Web Frontend
- [ ] Notificaciones en tiempo real

---

## 📄 Licencia

MIT

---

## 👨‍💻 Autor

Equipo de desarrollo - https://github.com/team-rust

  
- **`domain/services/`**: Servicios de dominio
  - `PathAnalyzer`: Contiene la lógica principal: contar archivos y carpetas

#### 2. **Application (Casos de Uso)**
Orquesta la lógica del dominio para implementar casos de uso específicos.

- **`application/use_cases/`**
  - `AnalyzePathUseCase`: Coordina el análisis de una ruta usando el `PathAnalyzer`

#### 3. **Adapters (Interfaces)**
Conecta el dominio con el mundo exterior. Implementa los puertos del dominio.

- **`adapters/input/`** - Puerto de entrada (lado izquierdo)
  - `cli.rs`: Parsea argumentos de línea de comandos
  - `cli_output.rs`: Formatea la salida para CLI
  
- **`adapters/output/`** - Puerto de salida (lado derecho)
  - `file_system.rs`: Implementación real de `FileSystemPort` usando el filesystem del SO

#### 4. **Infrastructure (Configuración)**
Inyección de dependencias y configuración general.

- **`infrastructure/bootstrap.rs`**: Arma la aplicación, conectando adaptadores con servicios

#### 5. **main.rs**
Punto de entrada que utiliza la infraestructura para ejecutar la aplicación.

## 🔄 Flujo de Datos (Hexágono)

```
┌─────────────────────────────────────────────────────────────┐
│                     MUNDO EXTERNO                           │
│  (CLI Input) ────────────────────────── (Filesystem Output) │
└────────┬──────────────────────────────────────┬─────────────┘
         │                                      │
         ▼                                      ▼
    ┌─────────────┐                    ┌──────────────────┐
    │ CLI Adapter │                    │ FileSystem       │
    │ (input/)    │                    │ Adapter (out/)   │
    └──────┬──────┘                    └────────┬─────────┘
           │                                    │
           │   Implementa Puerto                │
           └────────────┬───────────────────────┘
                        │
                        ▼
            ┌──────────────────────┐
            │   FileSystemPort     │
            │    (Interfaz)        │
            └──────────┬───────────┘
                       │
                       ▼
         ┌─────────────────────────────┐
         │    PathAnalyzer Service     │
         │   (Dominio - Lógica Core)   │
         └─────────────────────────────┘
                       │
                       ▼
         ┌─────────────────────────────┐
         │    AnalyzePathUseCase       │
         │   (Caso de Uso)             │
         └─────────────────────────────┘
                       │
                       ▼
         ┌──────────────────────────────┐
         │     PathStats (Entidad)      │
         │   (Modelo de Dominio)        │
         └──────────────────────────────┘
```

## 🎯 Principios de Arquitectura Hexagonal

1. **Independencia de Frameworks**: La lógica de negocio no depende de bibliotecas externas
2. **Testabilidad**: Fácil crear mocks de los puertos para testing
3. **Flexibilidad**: Cambiar de CLI a WebAPI o de filesystem a base de datos sin tocar el núcleo
4. **Claridad**: Las responsabilidades están bien definidas en cada capa

## 📦 Compilación

```bash
# Compilar en modo debug
cargo build

# Compilar en modo release (optimizado)
cargo build --release
```

## 🚀 Uso

```bash
# Analizar la carpeta actual
cargo run -- .

# Analizar una carpeta específica
cargo run -- /ruta/a/analizar

# Ejecutable compilado (después de compilar)
./target/debug/chat .
./target/release/chat /ruta/específica
```

## 📊 Ejemplo de Salida

```
Analizando ruta: /home/usuario/proyecto

📂 Carpeta: /home/usuario/proyecto
├ 📄 Archivos: 42
└ 📁 Carpetas: 15
```

## 🧪 Testing

Para crear tests que validen la lógica sin acceder al filesystem real:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Mock del FileSystemPort
    struct MockFileSystem {
        // implementación de prueba
    }

    #[test]
    fn test_analyze_path() {
        // El puerto permite inyectar una implementación de prueba
    }
}
```

## 📚 Referencias de Arquitectura Hexagonal

- **Puertos**: Interfaces que definen cómo interactúa el dominio con el exterior
- **Adaptadores**: Implementaciones concretas de los puertos
- **Lado Izquierdo**: Adaptadores de entrada (eventos, UI, API)
- **Lado Derecho**: Adaptadores de salida (base de datos, sistemas externos)
- **Núcleo (Domain)**: Lógica de negocio pura e independiente

## 📄 Estructura de Carpetas Detallada

| Carpeta | Responsabilidad |
|---------|-----------------|
| `domain/` | Lógica de negocio independiente |
| `domain/models/` | Entidades y value objects |
| `domain/ports/` | Interfaces que definen contratos |
| `domain/services/` | Lógica de dominio orquestada |
| `application/` | Casos de uso |
| `adapters/input/` | Puertos de entrada (CLI, API, etc.) |
| `adapters/output/` | Puertos de salida (DB, Filesystem, etc.) |
| `infrastructure/` | Configuración e inyección de dependencias |

## 💡 Beneficios de esta Arquitectura

✅ **Bajo Acoplamiento**: El dominio no conoce los adaptadores
✅ **Alta Cohesión**: Cada módulo tiene una responsabilidad clara
✅ **Fácil Testing**: Los puertos permiten crear mocks
✅ **Flexible**: Cambiar implementaciones sin afectar el core
✅ **Escalable**: Agregar nuevo adaptadores fácilmente
✅ **Mantenible**: Código organizado y predecible

---

**Arquitectura Hexagonal en Rust** | Proyecto: Chat | Versión 1.0
