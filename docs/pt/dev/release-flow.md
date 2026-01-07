# Fluxo de Lançamento (Release Flow)

Este documento descreve o processo de lançamento para o archlint.

## Visão Geral

O archlint usa o **semantic-release** para automatizar todo o workflow de lançamento. Os números de versão são calculados com base nas mensagens de commit seguindo o formato Conventional Commits.

## Formato de Mensagem de Commit

Todos os commits **devem** seguir o formato Conventional Commits. Isso é imposto pelo commitlint no CI/CD.

### Formato

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Tipos

| Tipo       | Descrição               | Incremento de Versão |
| ---------- | ----------------------- | -------------------- |
| `feat`     | Nova funcionalidade     | **Minor** (0.x.0)    |
| `fix`      | Correção de bug         | **Patch** (0.0.x)    |
| `perf`     | Melhoria de performance | **Patch** (0.0.x)    |
| `refactor` | Refatoração de código   | Nenhuma              |
| `docs`     | Documentação            | Nenhuma              |
| `test`     | Testes                  | Nenhuma              |
| `chore`    | Manutenção              | Nenhuma              |
| `ci`       | Alterações de CI/CD     | Nenhuma              |
| `build`    | Sistema de build        | Nenhuma              |

### Mudanças Quebrantes (Breaking Changes)

Adicione `!` após o tipo ou `BREAKING CHANGE:` no rodapé para disparar um incremento de versão **major**:

```bash
# Incremento de versão major (1.0.0)
git commit -m "feat!: change API signature"

# Ou
git commit -m "feat: new feature

BREAKING CHANGE: This changes the public API"
```

## Processo de Lançamento

### 1. Desenvolvimento

Desenvolva funcionalidades em branches de feature e faça o merge no `main`.

### Branches de Pré-lançamento (Prerelease)

O arquivo `.releaserc.json` contém configurações estáticas de branch para os canais `beta` e `alpha`. No entanto, **as branches de pré-lançamento são configuradas dinamicamente pelo CI/CD** durante o workflow de lançamento. O workflow cria automaticamente configurações de branch com base no canal selecionado e no nome do branch atual, portanto as entradas estáticas no `.releaserc.json` não são usadas durante os lançamentos reais.

### 2. Disparar Lançamento

Quando estiver pronto para lançar, dispare manualmente o workflow de Release:

1. Vá para **Actions** -> workflow de **Release**.
2. Clique em **Run workflow**.
3. (Opcional) Defina `dry_run` como `true` para ver o que aconteceria sem publicar de fato.

### 3. Etapas Automáticas

O workflow irá:

1. **Calcular Versão**: `semantic-release` analisa os commits desde o último lançamento.
2. **Atualizar Arquivos**: Atualiza automaticamente `Cargo.toml`, `package.json` e `CHANGELOG.md`.
3. **Commit & Tag**: Cria um novo commit e uma tag Git para o lançamento.
4. **Disparar CI/CD**: O push da tag dispara o workflow de CI, que constrói todos os binários.
5. **Publicar no npm**: O CI publica todos os pacotes no registro npm (apenas em tags).
6. **Anexar Binários**: O CI faz o upload dos binários independentes para o GitHub Release.

## Números de Versão

Todos os pacotes compartilham a mesma versão (versionamento unificado):

- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- etc.

## Verificando o Status do Lançamento

### Ver o Status do Workflow

https://github.com/archlinter/archlint/actions

### Verificar Publicação no npm

```bash
npm view @archlinter/cli
```

### Testar Instalação

```bash
npx @archlinter/cli@latest --version
```

## Solução de Problemas

### Commit Rejeitado pelo commitlint

**Correção**: Siga o formato de conventional commits:

```bash
git commit --amend -m "feat: correct commit message"
```

### Workflow de Lançamento Falhou

Verifique:

1. Segredo NPM_TOKEN configurado?
2. Segredo GH_PAT configurado?
3. O build do CI falhou?

## Referência

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [semantic-release](https://github.com/semantic-release/semantic-release)
