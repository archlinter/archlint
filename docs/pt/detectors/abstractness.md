---
title: Violação de Abstração
description: 'Detecta módulos que são muito concretos e estáveis (difíceis de mudar) ou muito abstratos e instáveis (sobreengenharia), violando o Princípio de Abstrações Estáveis.'
---

# Violação de Abstração (Abstractness Violation)

**ID:** `abstractness` | **Gravidade:** Média (padrão)

## O que esta regra detecta (TL;DR)

Esta regra marca módulos que são:

- **Muito concretos e muito estáveis** — muitos arquivos dependem de uma classe concreta (difícil de mudar com segurança).
- **Muito abstratos e muito instáveis** — abstrações das quais ninguém depende (sobreengenharia/YAGNI).

Em resumo:
👉 **Módulos estáveis** (a base) devem ser **abstratos**.
👉 **Módulos instáveis** (as folhas) devem ser **concretos**.

---

## O que é um "Módulo"?

Em `archlint`, um **módulo** é um único arquivo fonte (`.ts`, `.tsx`, etc.) que exporta pelo menos um símbolo.

- Cada arquivo é analisado independentemente.
- Arquivos barrel (`index.ts`) são tratados como módulos de agregação.
- Reexportações e importações são contadas como dependências entre módulos.

---

## Métricas e Intuição

### 1. Instabilidade (I)

Mede o quão propenso um módulo é a mudanças.

`I = Acoplamento Eferente (Ce) / (Acoplamento Aferente (Ca) + Acoplamento Eferente (Ce))`

**Intuição**:

- **Estável (I ≈ 0)**: Muitos arquivos te importam, mas você quase não importa ninguém. Você é um componente fundamental.
- **Instável (I ≈ 1)**: Você importa muitas coisas, mas ninguém te importa. Você está na borda do sistema.

### 2. Abstração (A)

Usamos um **Cálculo Semântico** baseado no uso real:

`A = (Clientes que importam apenas Interfaces/Tipos) / (Total de Clientes)`

**Diferença importante do A clássico**:
**NÃO** apenas contamos palavras-chave ou interfaces dentro do arquivo. Em vez disso, medimos **como o módulo é realmente usado**:

- Importar uma `class` concreta → **dependência concreta**.
- Importar uma `interface` ou `type` → **dependência abstrata**.

Isso reflete o _acoplamento arquitetural real_, não apenas a sintaxe. Usar `import type` é um sinal forte de intenção abstrata.

### 3. Distância (D)

A distância da linha ideal da "Sequência Principal" onde `A + I = 1`.

`D = |A + I - 1|`

---

## Zonas de Risco (Interpretação)

Com base nos valores de **A** e **I**, os módulos caem em zonas específicas:

### 🧱 Zona de Dor

- **Métricas**: I ≈ 0–0.3 (estável), A ≈ 0–0.3 (concreto).
- **Problema**: Todos dependem de uma implementação concreta. Mudá-la é perigoso porque é tanto rígida quanto altamente acoplada.

**Exemplo Ruim (Dependência concreta):**

```typescript
// database.service.ts
export class DatabaseService {
  save(data: any) {
    /* lógica concreta */
  }
}

// client.ts (100+ arquivos fazendo isso)
import { DatabaseService } from './database.service'; // Importação direta da classe
const db = new DatabaseService();
```

**Por que é marcado**:

- `Ca` = 100+ (muito estável, `I ≈ 0`).
- `A` = 0 (clientes importam a classe diretamente).
- `D` ≈ 1 → Distância máxima da sequência principal.

### 💨 Zona de Inutilidade

- **Métricas**: I ≈ 0.7–1.0 (instável), A ≈ 0.7–1.0 (abstrato).
- **Problema**: Abstrações sobreengenheiradas que ninguém usa.

**Exemplo:**

```typescript
// complex-plugin.interface.ts
export interface IComplexPlugin {
  execute(context: any): Promise<void>;
}
// 0 implementações e 0 clientes usando esta interface.
```

**Por que é marcado**:

- `I` ≈ 1 (ninguém depende dele).
- `A` = 1 (é puramente abstrato).
- `D` ≈ 1 → A abstração existe sem propósito.

---

## Heurísticas para Reduzir Falsos Positivos

A análise estática pode ser ruidosa. Essas heurísticas focam a regra em **decisões arquiteturais**, não em código incidental:

1.  **Limiar de Estabilidade (Fan-in)**: Apenas módulos com pelo menos `fan_in_threshold` (padrão: 10) dependentes são analisados. Se apenas alguns arquivos usam um módulo, seu impacto arquitetural é baixo.
2.  **DTOs e Entidades**: Classes sem métodos (apenas dados) são ignoradas. Elas são **transportadores de dados**, não componentes comportamentais.
3.  **Erros**: Classes que estendem `Error` são ignoradas. Elas são **sempre concretas por design**.
4.  **Scripts de Infraestrutura**: Migrações de banco de dados (`up`/`down`) são ignoradas, pois são **scripts procedimentais**, não parte da arquitetura de longo prazo.

---

## Como Corrigir (Guia de Decisão)

1. **O módulo é estável (tem muitos dependentes)?**
   - **Sim**: Extraia uma `interface`. Certifique-se de que os clientes usem `import type { ... }`. Use Injeção de Dependências.
   - **Não**: As abstrações podem ser desnecessárias. Mantenha-o concreto até que a estabilidade aumente.

2. **Há mais de uma implementação?**
   - **Não**: Se for instável, considere remover a interface (YAGNI).
   - **Sim**: A interface é justificada, mas certifique-se de que os clientes dependam da interface, não das classes.

---

## Configuração

```yaml
rules:
  abstractness:
    severity: medium
    distance_threshold: 0.85 # Limiar de ativação para a distância D
    fan_in_threshold: 10 # Mínimo de dependências entrantes (Fan-in) para ativar a análise
```
