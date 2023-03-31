package main

import (
    "os"
    "fmt"
    "time"
    "encoding/binary"
    ipns "github.com/ipfs/go-ipns"
    crypto "github.com/libp2p/go-libp2p-core/crypto"
    proto "github.com/golang/protobuf/proto"
)

func main() {
	fmt.Println("-----------------------------GenerateKey RSA Pair-----------------------------------------")
        var duration time.Duration = 60000000000
        
	privateKey, publicKey, err := crypto.GenerateKeyPair(crypto.RSA, 2048)
	if err != nil {
		panic(err)
	}
	ipnsRecord, err := ipns.Create(privateKey, []byte("/ipfs/QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u"), 0, time.Now().Add(24*time.Hour), duration)	
	if err != nil {
		panic(err)
	}
        ipns.EmbedPublicKey(publicKey,ipnsRecord)
	fmt.Println("------------------------------------------------------------------------------------------")        
	fmt.Println("-----------------------------Validate IPNS record-----------------------------------------")        
      	res := ipns.Validate(publicKey,ipnsRecord)
	fmt.Println(res)
	fmt.Println("------------------------------------------------------------------------------------------")	
	fmt.Println("------------------------------Saved IPNS Record-------------------------------------------")
	data, err := proto.Marshal(ipnsRecord)
	if err != nil {
		panic(err)
	}
       	file, err := os.Create("signed-ipns-record.bin")
	if err != nil {
		fmt.Println("Couldn't open file")
	}
	err = binary.Write(file, binary.LittleEndian, data)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println("------------------------------------------------------------------------------------------")	
	fmt.Println("-------------------------------Save Private Key-------------------------------------------")
	privateKeyData, err := crypto.MarshalPrivateKey(privateKey)
	if err != nil {
		panic(err)
	}
       	fileKeyData, err := os.Create("private-key-ipns-record.bin")
	if err != nil {
		fmt.Println("Couldn't open file")
	}
	err = binary.Write(fileKeyData, binary.LittleEndian, privateKeyData)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println("------------------------------------------------------------------------------------------")	
}
