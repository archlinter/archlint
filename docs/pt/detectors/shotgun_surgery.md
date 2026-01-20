# Cirurgia por Difusão (Shotgun Surgery)

**ID:** `shotgun_surgery` | **Gravidade:** Medium (default)

A cirurgia por difusão é aquela situação chata onde uma mudança "simples" exige que você toque em 15 arquivos diferentes. É como tentar consertar um vazamento tampando cem buraquinhos em vez de trocar o cano.

## Por que isso é um smell

- **Alta fricção**: Cada pequena mudança de requisito se torna uma operação grande.
- **Fácil de esquecer um lugar**: Quando a lógica está espalhada por todo lado, é quase certo que você vai esquecer de atualizar um desses arquivos, levando a "bugs fantasma".
- **Encapsulamento quebrado**: É sinal de que uma única responsabilidade escapou do seu módulo e agora está escondida em cada canto do seu código.

## Como corrigir

- **Consolidar Responsabilidades**: Use **Move Method** ou **Move Field** para trazer a lógica relacionada para um único módulo.
- **Introduzir Objeto de Parâmetro**: Se múltiplos módulos requerem o mesmo conjunto de dados, agrupe-o em um único objeto.
- **Substituir Valor por Objeto**: Se você tem muitos módulos manipulando os mesmos dados primitivos, encapsule esses dados e seu comportamento em uma nova classe.

## Configuração

```yaml
rules:
  shotgun_surgery:
    severity: medium
```
