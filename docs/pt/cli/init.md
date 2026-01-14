# archlint init

O comando `init` ajuda você a configurar rapidamente o archlint em um novo projeto, gerando um arquivo de configuração.

## Uso

```bash
archlint init [options]
```

## Opções

| Opção              | Padrão  | Descrição                                                     |
| ------------------ | ------- | ------------------------------------------------------------- |
| `-f, --force`      | `false` | Sobrescreve o arquivo `.archlint.yaml` se ele existir         |
| `--no-interactive` | `false` | Pula a seleção interativa de frameworks                       |
| `--presets <list>` | `none`  | Especifica explicitamente os presets (separados por vírgulas) |

## Como Funciona

1. **Detecção de Frameworks**: o archlint analisa o seu `package.json` e a estrutura do projeto para detectar os frameworks usados.
2. **Seleção Interativa**: A menos que `--no-interactive` seja usado, ele solicitará que você confirme ou selecione presets adicionais.
3. **Geração de Configuração**: Cria um arquivo `.archlint.yaml` com os presets selecionados e uma referência ao esquema JSON para suporte no IDE.

## Exemplos

### Inicialização interativa

```bash
archlint init
```

### Inicialização não interativa com presets específicos

```bash
archlint init --no-interactive --presets nestjs,prisma
```

### Sobrescrever configuração existente

```bash
archlint init --force
```
