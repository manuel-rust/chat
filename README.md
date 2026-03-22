# Agenda - Analizador de Rutas con Arquitectura Hexagonal

Aplicación Rust que analiza rutas del sistema de archivos, contando archivos y carpetas. Implementa una **arquitectura hexagonal (puertos y adaptadores)** para separar la lógica de negocio de las interfaces externas.

## 📋 Descripción

Este proyecto cuenta recursivamente el número de archivos y carpetas dentro de una ruta especificada por línea de comandos. La arquitectura hexagonal permite que la lógica de negocio sea independiente de la forma en que se reciben los datos (CLI) o se persisten (filesystem).

## 🏗️ Arquitectura Hexagonal

### Estructura del Proyecto

```
src/
├── main.rs                          (Punto de entrada)
├── lib.rs                           (Biblioteca exportada)
│
├── domain/                          (Núcleo - Lógica de negocio)
│   ├── models/
│   │   └── path_stats.rs           (Entidad del dominio - datos puros)
│   ├── ports/
│   │   └── file_system_port.rs     (Puerto - interfaz agnóstica)
│   └── services/
│       └── path_analyzer.rs        (Servicio de dominio - lógica core)
│
├── application/                     (Casos de uso)
│   └── use_cases/
│       └── analyze_path.rs         (Orquestación de la lógica)
│
├── adapters/                        (Implementaciones - Lado Izquierdo y Derecho)
│   ├── input/
│   │   ├── cli.rs                  (Adaptador CLI - entrada de usuario)
│   │   └── cli_output.rs           (Formateador CLI)
│   └── output/
│       └── file_system.rs          (Adaptador Filesystem - implementación del puerto)
│
└── infrastructure/
    └── bootstrap.rs                (Configuración e inyección de dependencias)
```

### Capas de la Arquitectura

#### 1. **Domain (Núcleo)**
La capa más interna e independiente. Contiene la lógica de negocio pura sin dependencias externas.

- **`domain/models/`**: Entidades del dominio
  - `PathStats`: Estructura que representa las estadísticas de una ruta (archivos, carpetas)
  
- **`domain/ports/`**: Puertos (interfaces)
  - `FileSystemPort`: Define cómo debe funcionar la lectura del filesystem (agnóstico de la implementación)
  
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
./target/debug/agenda .
./target/release/agenda /ruta/específica
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

**Arquitectura Hexagonal en Rust** | Proyecto: Agenda | Versión 1.0
