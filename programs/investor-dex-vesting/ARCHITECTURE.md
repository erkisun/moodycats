## 🏗️ ARCHITEKTUR-DIAGRAMM

```mermaid

graph TD
    %% Entrypoint
    Lib[lib.rs]

    %% Instructions Ordner
    subgraph Instructions [Ordner: instructions/]
        I1[initialize.rs]
        I2[release_dex.rs]
        I3[register_investor.rs]
        I4[claim_starter.rs]
        I5[earlybird_bonus.rs]
        I6[dev_allocation.rs]
        I7[revoke_admin.rs]
    end

    %% State / On-Chain Struktur
    subgraph States [Ordner: states/]
        Config[config.rs <br/><i>On-Chain Storage (Accounts)</i>]
    end

    %% Verbindungen
    Lib --> I1
    Lib --> I2
    Lib --> I3
    Lib --> I4
    Lib --> I5
    Lib --> I6
    Lib --> I7

    %% Alle Instructions nutzen die Config
    I1 -.-> Config
    I2 -.-> Config
    I3 -.-> Config
    I4 -.-> Config
    I5 -.-> Config
    I6 -.-> Config
    I7 -.-> Config

    %% Styling (Optional)
    style Config fill:#f9f,stroke:#333,stroke-width:4px
    style Lib fill:#bbf,stroke:#333

```
