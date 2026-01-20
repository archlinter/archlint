# Inveja de Recursos

**ID:** `feature_envy` | **Gravidade:** Medium (default)

A inveja de recursos é como aquele vizinho bisbilhoteiro que sabe mais sobre o que acontece na sua casa do que você mesmo. Acontece quando um método parece muito mais interessado nos dados de outra classe do que nos seus próprios.

## Por que isso é um smell

É um sinal clássico de lógica mal posicionada. Se um método está constantemente mexendo em outro objeto para puxar dados e fazer cálculos, essa lógica provavelmente pertence dentro do outro objeto. Quebra o encapsulamento e deixa suas classes fortemente acopladas.

## Como corrigir

Mova o método (ou a parte do método que possui a inveja) para a classe cujos dados ele está usando.
