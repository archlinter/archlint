# Cirurgia por Difusão (Shotgun Surgery)

**ID:** `shotgun_surgery` | **Gravidade:** Medium (default)

A cirurgia por difusão ocorre quando uma única mudança em seus requisitos exige que você faça muitas pequenas alterações em muitos módulos diferentes. O `archlint` detecta isso analisando o histórico do git para encontrar arquivos que mudam frequentemente juntos (alta frequência de co-mudança).

## Por que isso é um smell

- **Alto Custo de Manutenção**: Cada feature ou correção de bug requer tocar em múltiplas partes do sistema.
- **Propenso a Erros**: É fácil esquecer uma das muitas mudanças necessárias, levando a bugs.
- **Encapsulamento Deficiente**: Indica que uma única responsabilidade está fragmentada pela base de código ao invés de estar encapsulada em um só lugar.

## Como corrigir

- **Consolidar Responsabilidades**: Use **Move Method** ou **Move Field** para trazer a lógica relacionada para um único módulo.
- **Introduzir Objeto de Parâmetro**: Se múltiplos módulos requerem o mesmo conjunto de dados, agrupe-o em um único objeto.
- **Substituir Valor por Objeto**: Se você tem muitos módulos manipulando os mesmos dados primitivos, encapsule esses dados e seu comportamento em uma nova classe.

## Configuração

```yaml
rules:
  shotgun_surgery:
    severity: warn
```
