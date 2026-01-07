# Primeira Verificação

Uma vez instalado, executar sua primeira verificação é simples.

## Execute uma Verificação Básica

Navegue até a raiz do seu projeto e execute:

```bash
npx @archlinter/cli scan
```

Por padrão, o archlint irá:

1. Verificar todos os arquivos TypeScript e JavaScript no diretório atual.
2. Respeitar o seu arquivo `.gitignore`.
3. Usar limites padrão para todos os mais de 28 detectores.
4. Exibir um resumo em tabela colorida dos code smells detectados.

## Salvar um Snapshot

Para usar a abordagem "Catraca", você primeiro precisa capturar o estado atual da sua arquitetura:

```bash
npx @archlinter/cli snapshot -o .archlint-baseline.json
```

Este arquivo representa sua linha de base (baseline) arquitetural. Você deve commitá-lo em seu repositório.

## Verificar Regressões

Agora, conforme você desenvolve, pode verificar se suas alterações introduziram novos problemas arquiteturais:

```bash
npx @archlinter/cli diff .archlint-baseline.json
```

Em um ambiente de CI, você normalmente compararia com a branch principal:

```bash
npx @archlinter/cli diff origin/main --fail-on medium
```

## O que vem a seguir?

- [Aprenda sobre todos os Detectores](/pt/detectors/)
- [Configurar archlint.yaml](/pt/configuration/)
- [Integrar no CI/CD](/pt/integrations/github-actions)
