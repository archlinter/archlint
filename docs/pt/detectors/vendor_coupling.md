# Acoplamento de Fornecedor (Vendor Coupling)

**ID:** `vendor_coupling` | **Gravidade:** Medium (default)

Identifica módulos que ficaram "casados" com uma biblioteca ou framework externo específico.

## Por que isso é um smell

- **Vendor lock-in**: Se aquela biblioteca ficar depreciada ou você decidir trocar para uma alternativa melhor, vai ter que reescrever metade do seu código.
- **Fricção nos testes**: Você não consegue testar sua lógica de negócio sem também puxar a biblioteca externa pesada e seus mocks.
- **Difícil de atualizar**: Você fica preso na versão que a biblioteca suporta porque ela está entrelaçada em cada arquivo.

## Como corrigir

Use o **Padrão Adapter**. Crie uma interface em seu domínio e implemente-a usando a biblioteca externa. O restante do seu código deve depender apenas da sua interface.

## Configuração

```yaml
rules:
  vendor_coupling:
    severity: medium
    max_files_per_package: 10
    ignore_packages:
      - 'lodash'
      - 'rxjs'
      - '@nestjs/*'
```

### Opções

- `max_files_per_package` (padrão: 10): O número máximo de arquivos que podem importar um pacote específico antes que um smell seja relatado.
- `ignore_packages`: Uma lista de nomes de pacotes ou padrões glob para ignorar.
