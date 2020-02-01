from ecies.utils import generate_eth_key, generate_key
from ecies import encrypt, decrypt

eth_key = generate_eth_key()
sk_hex = eth_key.to_hex()
pk_hex = eth_key.public_key.to_hex()
data = b'neko is kawaii'

# encrypt
enc_data = encrypt(pk_hex, data)
print(data, "\n\033[31m^ encrypt to ->\n\033[m   \033[33m",  enc_data, "\033[m")

print("\n--------------------------------------------------\n")

# decrypt
dec_data = decrypt(sk_hex, enc_data)
print(enc_data, "\n\033[31m^ decrypt to ->\n\033[m   \033[33m", dec_data, "\033[m")

