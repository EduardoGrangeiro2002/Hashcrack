// Responsabilidades =>
// Será responsável por workestrar o grupo de threads onde se comunicará via canais para enviar informações
// As informações são para parar, novas senhas, worker livre e quando encontrar a senha
// Quando encontrar a senha ele envia por canal uma ordem de pausa e para todos os workers.
// Caso ele receba a informação de que tem um worker livre ele enviar mais senhas se tiver, se não tiver ele manda uma ordem de pausa.