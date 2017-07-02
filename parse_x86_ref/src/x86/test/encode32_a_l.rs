use x86;
use x86::{Mnemonic, Operand, Reg, OperandSize, SegmentReg};
use x86::test::{encode32_helper0, encode32_helper1, encode32_helper1, encode32_helper3, encode32_helper2, encode32_helper3, test_aliased};

#[test]
fn instr_aaa() {
    x86::load(); // This is kind of a hack to load the instruction definitions, maybe should lazy load them or use a real test framework?
    encode32_helper0(Mnemonic::AAA, &vec![0x37]); // AAA
}

#[test]
fn instr_() {
    // TODO Defs are wrong
    // encode32_helper0(Mnemonic::AAD, &vec![0xD5, 0x0A]); // AAD
    // encode32_helper1(Mnemonic::AAD, Operand::Literal8(0x3), &vec![0xD5, 0x03]); // AAD 3
}

#[test]
fn instr_aam() {
    // TODO Defs are wrong
    // encode32_helper0(Mnemonic::AAM, &vec![0xD4, 0x0A]); // AAM
    // encode32_helper1(Mnemonic::AAM, Operand::Literal8(0x3), &vec![0xD4, 0x03]); // AAM 3
}

#[test]
fn instr_aas() {
   encode32_helper0(Mnemonic::AAS, &vec![0x3F]); // AAS
}

#[test]
fn instr_adc() {
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x14, 0x12]); // ADC AL, 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x15, 0x34, 0x12]); // ADC AX, 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x15, 0x78, 0x56, 0x34, 0x12]); // ADC EAX, 0x12345678
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x10, 0x12]); // ADC BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0x80, 0xD3, 0x12]); // ADC BL, 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x10, 0x34, 0x12]); // ADC WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0xd3, 0x34, 0x12]); // ADC BX, 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x10, 0x78, 0x56, 0x34, 0x12]); // ADC WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EBX), Operand::Literal32(0x12345678), &vec![0x81, 0xd3, 0x78, 0x56, 0x34, 0x12]); // ADC EBX, 0x12345678
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x10, 0x12]); // ADC WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Literal8(0x12), &vec![0x66, 0x83, 0xd3, 0x12]); // ADC BX, 0x12
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x10, 0x18]); // ADC BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BL), Operand::Direct(Reg::CL), &vec![0x10, 0xCB]); // ADC BL, CL
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x11, 0x18]); // ADC WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x11, 0xcb]); // ADC BX, CX
    encode32_helper2(Mnemonic::ADC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x11, 0x18]); // ADC DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x11, 0xcb]); // ADC EBX, ECX
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x12, 0x18]); // ADC BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x13, 0x18]); // ADC BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::ADC, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x13, 0x18]); // ADC EBX, DWORD PTR [EAX]
}

#[test]
fn instr_adx() {
    // TODO No nmemonic/def
    // panic!("TODO");
}

#[test]
fn instr_add() {
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x04, 0x12]); // ADD AL, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x05, 0x34, 0x12]); // ADD AX, 0x1234
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x05, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, 0x12345678
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0x80, 0xC3, 0x12]); // ADD BL, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x00, 0x12]); // ADD BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0xC3, 0x34, 0x12]); // ADD BX, 0x1234
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x00, 0x34, 0x12]); // ADD WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Literal32(0x12345678), &vec![0x81, 0xC3, 0x78, 0x56, 0x34, 0x12]); // ADD EBX, 0x12345678
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x00, 0x78, 0x56, 0x34, 0x12]); // ADD DWORD PTR [EAX], 0x12345678
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Literal8(0x12), &vec![0x66, 0x83, 0xC3, 0x12]); // ADD BX, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x00, 0x12]); // ADD WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Literal8(0x12), &vec![0x83, 0xC3, 0x12]); // ADD EBX, 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x83, 0x00, 0x12]); // ADD DWORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BL), Operand::Direct(Reg::CL), &vec![0x00, 0xCB]); // ADD BL, CL
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x00, 0x18]); // ADD BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x01, 0xCB]); // ADD BX, CX
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x01, 0x18]); // ADD WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x01, 0xCB]); // ADD EBX, ECX
    encode32_helper2(Mnemonic::ADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x01, 0x18]); // ADD DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x02, 0x18]); // ADD BL, BYTE PTR [EAX]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x03, 0x18]); // ADD BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x03, 0x18]); // ADD EBX, DWORD PTR [EAX]
}


#[test]
fn instr_addpd() {
    // panic!("TODO");
}

#[test]
fn instr_addps() {
    // panic!("TODO");
}

#[test]
fn instr_addsd() {
    // panic!("TODO");
}

#[test]
fn instr_addss() {
    // panic!("TODO");
}

#[test]
fn instr_addsubpd() {
    // panic!("TODO");
}

#[test]
fn instr_addsubps() {
    // panic!("TODO");
}

#[test]
fn instr_adox() {
    // panic!("TODO");
}

#[test]
fn instr_aesdec() {
    // panic!("TODO");
}

#[test]
fn instr_aesdeclast() {
    // panic!("TODO");
}

#[test]
fn instr_aesenc() {
    // panic!("TODO");
}

#[test]
fn instr_aesenclast() {
    // panic!("TODO");
}

#[test]
fn instr_aesimc() {
    // panic!("TODO");
}

#[test]
fn instr_aeskeygenassist() {
    // panic!("TODO");
}

#[test]
fn instr_and() {
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x24, 0x12]); // ADD AL, 0x12
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x25, 0x34, 0x12]); // ADD AX, 0x1234
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x25, 0x78, 0x56, 0x34, 0x12]); // ADD EAX, 0x12345678
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Literal8(0x12), &vec![0x80, 0x20, 0x12]); // ADD BYTE PTR [EAX], 0x12
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0x20, 0x34, 0x12]); // ADD WORD PTR [EAX], 0x1234
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x81, 0x20, 0x78, 0x56, 0x34, 0x12]); // ADD DWORD PTR [EAX], 0x12345678
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x83, 0x20, 0x12]); // ADD WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x83, 0x20, 0x12]); // ADD DWORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x20, 0x18]); // ADD BYTE PTR [EAX], BL
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x21, 0x18]); // ADD WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::AND, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x21, 0x18]); // ADD DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::BL), Operand::Direct(Reg::CL), &vec![0x20, 0xCB]); // ADD BL, CL
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::BX), Operand::Direct(Reg::CX), &vec![0x66, 0x21, 0xCB]); // ADD BX, CX
    encode32_helper2(Mnemonic::AND, Operand::Direct(Reg::EBX), Operand::Direct(Reg::ECX), &vec![0x21, 0xCB]); // ADD EBX, ECX
}

#[test]
fn instr_andn() {
    // panic!("TODO");
}

#[test]
fn instr_andpd() {
    // panic!("TODO");
}

#[test]
fn instr_andps() {
    // panic!("TODO");
}

#[test]
fn instr_andnpd() {
    // panic!("TODO");
}

#[test]
fn instr_andnps() {
    // panic!("TODO");
}

#[test]
fn instr_arpl() {
    encode32_helper3(Mnemonic::ARPL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x63, 0x18]); // ARPL WORD PTR [EAX], BX
}

#[test]
fn instr_blendpd() {
    // panic!("TODO");
}

#[test]
fn instr_bextr() {
    // panic!("TODO");
}

fn instr_plendps() {
    // panic!("TODO");
}

#[test]
fn instr_blendvpd() {
    // panic!("TODO");
}

#[test]
fn instr_blendvps() {
    // panic!("TODO");
}

#[test]
fn instr_blsi() {
    // panic!("TODO");
}

#[test]
fn instr_blsmsk() {
    // panic!("TODO");
}

#[test]
fn instr_blsr() {
    // panic!("TODO");
}

#[test]
fn instr_bndcl() {
    // panic!("TODO");
}

#[test]
fn instr_bndcu() {
    // panic!("TODO");
}

#[test]
fn instr_bndcn() {
    // panic!("TODO");
}

#[test]
fn instr_bndldx() {
    // panic!("TODO");
}

#[test]
fn instr_bndmk() {
    // panic!("TODO");
}

#[test]
fn instr_bndmov() {
    // panic!("TODO");
}

#[test]
fn instr_bndstx() {
    // panic!("TODO");
}

#[test]
fn instr_bound() {
    encode32_helper3(Mnemonic::BOUND, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x66, 0x62, 0x18]); // BOUND BX, DWORD PTR [EAX]
    encode32_helper3(Mnemonic::BOUND, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0x62, 0x18]); // BOUND EBX, QWORD PTR [EAX]
}

#[test]
fn instr_bsf() {
    encode32_helper2(Mnemonic::BSF, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0f, 0xbc, 0x18]); // BSF BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::BSF, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0f, 0xbc, 0x18]); // BSF EBX, DWORD PTR [EAX]
}

#[test]
fn instr_bsr() {
    encode32_helper2(Mnemonic::BSR, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0f, 0xbd, 0x18]); // BSF BX, WORD PTR [EAX]
    encode32_helper2(Mnemonic::BSR, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x0f, 0xbd, 0x18]); // BSF EBX, DWORD PTR [EAX]
}

#[test]
fn instr_bswap() {
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EAX), &vec![0x0f, 0xc8]); // BSWAP EAX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EBX), &vec![0x0f, 0xcb]); // BSWAP EBX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::ECX), &vec![0x0f, 0xc9]); // BSWAP ECX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EDX), &vec![0x0f, 0xca]); // BSWAP EDX
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EDI), &vec![0x0f, 0xcf]); // BSWAP EDI
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::ESI), &vec![0x0f, 0xce]); // BSWAP ESI
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::ESP), &vec![0x0f, 0xcc]); // BSWAP ESP
    encode32_helper1(Mnemonic::BSWAP, Operand::Direct(Reg::EBP), &vec![0x0f, 0xcd]); // BSWAP EBP
}

#[test]
fn instr_bt() {
    encode32_helper3(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xA3, 0x18]); // BT WORD PTR [EAX], BX
    encode32_helper3(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xA3, 0x18]); // BT DWORD PTR [EAX], EBX
    encode32_helper3(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x20, 0x12]); // BT WORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::BT, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x20, 0x12]); // BT DWORD PTR [EAX], 0x12
}

#[test]
fn instr_btc() {
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xBB, 0x18]); // BTC WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xBB, 0x18]); // BTC DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x38, 0x12]); // BTC WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BTC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x38, 0x12]); // BTC DWORD PTR [EAX], 0x12
}

#[test]
fn instr_btr() {
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xB3, 0x18]); // BTR WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xB3, 0x18]); // BTR DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x30, 0x12]); // BTR WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BTR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x30, 0x12]); // BTR DWORD PTR [EAX], 0x12
}

#[test]
fn instr_bts() {
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xAB, 0x18]); // BTS WORD PTR [EAX], BX
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x0F, 0xAB, 0x18]); // BTS DWORD PTR [EAX], EBX
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x0F, 0xBA, 0x28, 0x12]); // BTS WORD PTR [EAX], 0x12
    encode32_helper2(Mnemonic::BTS, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x0F, 0xBA, 0x28, 0x12]); // BTS DWORD PTR [EAX], 0x12
}

#[test]
fn instr_bzhi() {
    // No def
    // panic!("TODO");
}

#[test]
fn instr_call() {
    // TODO It seems many assemblers factor in instruction size on relative call instructions.
    // Maybe we should do this?
    encode32_helper1(Mnemonic::CALL, Operand::Literal16(0x1234), &vec![0x66, 0xe8, 0x34, 0x12]); // CALL 0x1234
    encode32_helper1(Mnemonic::CALL, Operand::Literal32(0x12345678), &vec![0xe8, 0x78, 0x56, 0x34, 0x12]); // CALL 0x12345678
    encode32_helper1(Mnemonic::CALL, Operand::Direct(Reg::AX), &vec![0x66, 0xff, 0xd0]); // CALL AX
    encode32_helper1(Mnemonic::CALL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xff, 0x10]); // CALL WORD PTR [EAX]
    encode32_helper1(Mnemonic::CALL, Operand::Direct(Reg::EAX), &vec![0xff, 0xd0]); // CALL EAX
    encode32_helper1(Mnemonic::CALL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xff, 0x10]); // CALL DWORD PTR [EAX]
    // TODO Assembler should auto-detect far calls (alias callf?)
    encode32_helper1(Mnemonic::CALLF, Operand::MemoryAndSegment16(0x12, 0x3456), &vec![0x66, 0x9a, 0x56, 0x34, 0x12, 0x00]); // CALL 0x12:0x3456
    encode32_helper1(Mnemonic::CALLF, Operand::MemoryAndSegment32(0x12, 0x3456789a), &vec![0x9a, 0x9a, 0x78, 0x56, 0x34, 0x12, 0x00]); // CALL 0x12:0x3456789A
}

#[test]
fn instr_cbw() {
    encode32_helper0(Mnemonic::CBW, &vec![0x66, 0x98]); // CBW
}

#[test]
fn instr_cwde() {
    encode32_helper0(Mnemonic::CWDE, &vec![0x98]); // CWDE
}

#[test]
fn instr_clac() {
    // No def
    // panic!("TODO");
}

#[test]
fn instr_clc() {
    encode32_helper0(Mnemonic::CLC, &vec![0xf8]); // CLC
}

#[test]
fn instr_cld() {
    encode32_helper0(Mnemonic::CLD, &vec![0xfc]); // CLD
}

#[test]
fn instr_clflush() {
    encode32_helper1(Mnemonic::CLFLUSH, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x0f, 0xae, 0x38]); // CLFLUSH BYTE PTR [EAX]
}

#[test]
fn instr_clflushopt() {
    // No def
    // panic!("TODO");
}

#[test]
fn instr_cli() {
    encode32_helper0(Mnemonic::CLI, &vec![0xfa]); // CLI
}

#[test]
fn instr_clts() {
    encode32_helper0(Mnemonic::CLTS, &vec![0x0f, 0x06]); // CLTS
}

#[test]
fn instr_clwb() {
    // No def
    // panic!("TODO");
}

#[test]
fn instr_cmc() {
    encode32_helper0(Mnemonic::CMC, &vec![0xf5]); // CMC
}

#[test]
fn instr_cmov() {
    fn cmov_test_helper(mnemonic: Mnemonic, primary_opcode: u8) {
        encode32_helper2(mnemonic, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0x0f, primary_opcode, 0xc3]);
    }

    fn cmov_test_helper_aliased(mnemonics: &[Mnemonic], primary_opcode: u8) {
        test_aliased(mnemonics, |m| cmov_test_helper(m, primary_opcode) );
    }

    // OF
    cmov_test_helper(Mnemonic::CMOVO, 0x40);

    // !OF
    cmov_test_helper(Mnemonic::CMOVNO, 0x41);

    // CF
    cmov_test_helper_aliased(&[Mnemonic::CMOVB, Mnemonic::CMOVNAE, Mnemonic::CMOVC], 0x42);

    // !CF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNB, Mnemonic::CMOVAE, Mnemonic::CMOVNC], 0x43);
    
    // ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVZ, Mnemonic::CMOVE], 0x44);

    // !ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNZ, Mnemonic::CMOVNE], 0x45);
    
    // CF+ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVBE, Mnemonic::CMOVNA], 0x46);

    // !CF*!ZF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNBE, Mnemonic::CMOVA], 0x47);

    // SF
    cmov_test_helper(Mnemonic::CMOVS, 0x48);
    
    // !SF
    cmov_test_helper(Mnemonic::CMOVNS, 0x49);

    // PF
    cmov_test_helper_aliased(&[Mnemonic::CMOVP, Mnemonic::CMOVPE], 0x4A);

    // !PF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNP, Mnemonic::CMOVPO], 0x4B);

    // SF!=OF
    cmov_test_helper_aliased(&[Mnemonic::CMOVL, Mnemonic::CMOVNGE], 0x4C);

    // SF=OF
    cmov_test_helper_aliased(&[Mnemonic::CMOVNL, Mnemonic::CMOVGE], 0x4D);

    // ZF+(SF!=OF)
    cmov_test_helper_aliased(&[Mnemonic::CMOVLE, Mnemonic::CMOVNG], 0x4E);

    // !ZF*(SF=OF)
    cmov_test_helper_aliased(&[Mnemonic::CMOVNLE, Mnemonic::CMOVG], 0x4F);
}

#[test]
fn instr_cmp() {
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0x3C, 0x12]); // CMP AL, 0x12
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::AX), Operand::Literal16(0x1234), &vec![0x66, 0x3D, 0x34, 0x12]); // CMP AX, 0x1234
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::EAX), Operand::Literal32(0x12345678), &vec![0x3D, 0x78, 0x56, 0x34, 0x12]); // CMP EAX, 0x12345678
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::BL), Operand::Literal8(0x12), &vec![0x80, 0xFB, 0x12]); // CMP BL, 0x12
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::BX), Operand::Literal16(0x1234), &vec![0x66, 0x81, 0xFB, 0x34, 0x12]); // CMP BX, 0x1234
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::EBX), Operand::Literal32(0x12345678), &vec![0x81, 0xFB, 0x78, 0x56, 0x34, 0x12]); // CMP EAX, 0x12345678
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::BX), Operand::Literal8(0x12), &vec![0x66, 0x83, 0xFB, 0x12]); // CMP BX, 0x12
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::EBX), Operand::Literal8(0x12), &vec![0x83, 0xFB, 0x12]); // CMP EBX, 0x12
    encode32_helper3(Mnemonic::CMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), Operand::Direct(Reg::BL), &vec![0x38, 0x18]); // CMP BYTE PTR [EAX], BL
    encode32_helper3(Mnemonic::CMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Direct(Reg::BX), &vec![0x66, 0x39, 0x18]); // CMP WORD PTR [EAX], BX
    encode32_helper3(Mnemonic::CMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Direct(Reg::EBX), &vec![0x39, 0x18]); // CMP DWORD PTR [EAX], EBX
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::BL), Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0x3A, 0x18]); // CMP BL, BYTE PTR [EAX]
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x3B, 0x18]); // CMP BX, WORD PTR [EAX]
    encode32_helper3(Mnemonic::CMP, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0x3B, 0x18]); // CMP EBX, DWORD PTR [EAX]
}

#[test]
fn instr_cmppd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpsb() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpsw() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpsd_string() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpsd_sse() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpss() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cmpxchg() {
    encode32_helper2(Mnemonic::CMPXCHG, Operand::Direct(Reg::AL), Operand::Direct(Reg::BL), &vec![0x0F, 0xB0, 0xD8]); // CMPXCHG AL, BL
    encode32_helper2(Mnemonic::CMPXCHG, Operand::Direct(Reg::AX), Operand::Direct(Reg::BX), &vec![0x66, 0x0F, 0xB1, 0xD8]); // CMPXCHG AX, BX
    encode32_helper2(Mnemonic::CMPXCHG, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0x0F, 0xB1, 0xD8]); // CMPXCHG EAX, EBX
}

#[test]
fn instr_comisd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_comiss() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cpuid() {
    encode32_helper0(Mnemonic::CPUID, &vec![0x0f, 0xa2]); // CPUID
}

#[test]
fn instr_crc32() {
    encode32_helper2(Mnemonic::CRC32, Operand::Direct(Reg::EAX), Operand::Direct(Reg::BL), &vec![0xF2, 0x0F, 0x38, 0xF0, 0xC3]); // CRC32 EAX, BL
    encode32_helper2(Mnemonic::CRC32, Operand::Direct(Reg::EAX), Operand::Direct(Reg::BX), &vec![0x66, 0xF2, 0x0F, 0x38, 0xF1, 0xC3]); // CRC32 EAX, BX
    encode32_helper2(Mnemonic::CRC32, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX), &vec![0xF2, 0x0F, 0x38, 0xF1, 0xC3]); // CRC32 EAX, EBX
}

#[test]
fn instr_cvtdq2pd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtdq2ps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtpd2dq() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtpd2pi() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtpd2ps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtpi2pd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtpi2ps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtps2dq() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtps2pd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtps2pi() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtsd2si() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtsd2ss() {
    // TODO
    // panic!("TODO");
}
#[test]
fn instr_cvtsi2sd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtsi2ss() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtss2sd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvtss2si() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvttpd2dq() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvttpd2pi() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvttps2dq() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvttps2pi() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvttsd2si() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cvttss2si() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_cwd() {
    encode32_helper0(Mnemonic::CWD,  &vec![0x66, 0x99]); // CWD
}

#[test]
fn instr_cdq() {
    encode32_helper0(Mnemonic::CDQ,  &vec![0x99]); // CDQ
}

#[test]
fn instr_daa() {
    encode32_helper0(Mnemonic::DAA,  &vec![0x27]); // DAA
}

#[test]
fn instr_das() {
    encode32_helper0(Mnemonic::DAS,  &vec![0x2F]); // DAS
}

#[test]
fn instr_dec() {
    encode32_helper1(Mnemonic::DEC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xFE, 0x08]); // DEC BYTE PTR [EAX]
    encode32_helper1(Mnemonic::DEC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xFF, 0x08]); // DEC WORD PTR [EAX]
    encode32_helper1(Mnemonic::DEC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xFF, 0x08]); // DEC DWORD PTR [EAX]
    encode32_helper1(Mnemonic::DEC, Operand::Direct(Reg::BX), &vec![0x66, 0x4B]); // DEC BX
    encode32_helper1(Mnemonic::DEC, Operand::Direct(Reg::EBX), &vec![0x4B]); // DEC EBX
}


#[test]
fn instr_div() {
    encode32_helper1(Mnemonic::DIV, Operand::Direct(Reg::BL), &vec![0xF6, 0xF3]); // DIV BL
    encode32_helper1(Mnemonic::DIV, Operand::Direct(Reg::BX), &vec![0x66, 0xF7, 0xF3]); // DIV BX
    encode32_helper1(Mnemonic::DIV, Operand::Direct(Reg::EBX), &vec![0xF7, 0xF3]); // DIV EBX
}

#[test]
fn instr_divpd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_divps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_divsd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_divss() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_dppd() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_dpps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_emms() {
    encode32_helper0(Mnemonic::EMMS,  &vec![0x0F, 0x77]); // EMMS
}

#[test]
fn instr_enter() {
    encode32_helper3(Mnemonic::ENTER, Operand::Literal16(0x1234), Operand::Literal8(0x56), &vec![0xC8, 0x34, 0x12, 0x56]); // ENTER 0x1234, 0x56
}

#[test]
fn instr_extractps() {
    // TODO
    // panic!("TODO");
}

#[test]
fn instr_f2xm1() {
    encode32_helper0(Mnemonic::F2XM1,  &vec![0xD9, 0xF0]); // F2XM1
}

#[test]
fn instr_fabs() {
    encode32_helper0(Mnemonic::FABS,  &vec![0xD9, 0xE1]); // FABS
}

#[test]
fn instr_fadd() {
    encode32_helper1(Mnemonic::FADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x00]); // FADD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x00]); // FADD QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FADD, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xC2]); // FADD ST, ST(2)
    encode32_helper2(Mnemonic::FADD, Operand::Direct(Reg::ST4), Operand::Direct(Reg::ST), &vec![0xDC, 0xC4]); // FADD ST(4), ST
    encode32_helper2(Mnemonic::FADDP, Operand::Direct(Reg::ST3), Operand::Direct(Reg::ST), &vec![0xDE, 0xC3]); // FADDP ST(3), ST
    encode32_helper2(Mnemonic::FADDP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xC1]); // FADDP ST(1), ST
    encode32_helper1(Mnemonic::FIADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x00]); // FIADD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIADD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x00]); // FIADD WORD PTR [EAX]
}

#[test]
fn instr_fbld() {
    encode32_helper1(Mnemonic::FBLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDF, 0x20]); // FBLD TBYTE PTR [EAX]
}

#[test]
fn instr_fbstp() {
    encode32_helper1(Mnemonic::FBSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDF, 0x30]); // FBSTP TBYTE PTR [EAX]
}

#[test]
fn instr_fclex() {
    encode32_helper0(Mnemonic::FCLEX,  &vec![0x9B, 0xDB, 0xE2]); // FCLEX 
}

#[test]
fn instr_fnclex() {
    encode32_helper0(Mnemonic::FNCLEX,  &vec![0xDB, 0xE2]); // FNCLEX 
}

#[test]
fn instr_fcmov() {
    encode32_helper2(Mnemonic::FCMOVB, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1), &vec![0xDA, 0xC1]); // FCMOVB ST, ST(1)
    encode32_helper2(Mnemonic::FCMOVE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xDA, 0xCA]); // FCMOVE ST, ST(2)
    encode32_helper2(Mnemonic::FCMOVBE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST3), &vec![0xDA, 0xD3]); // FCMOVBE ST, ST(3)
    encode32_helper2(Mnemonic::FCMOVU, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST4), &vec![0xDA, 0xDC]); // FCMOVU ST, ST(4)
    encode32_helper2(Mnemonic::FCMOVNB, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST5), &vec![0xDB, 0xC5]); // FCMOVNB ST, ST(5)
    encode32_helper2(Mnemonic::FCMOVNE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST6), &vec![0xDB, 0xCE]); // FCMOVNE ST, ST(6)
    encode32_helper2(Mnemonic::FCMOVNBE, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST7), &vec![0xDB, 0xD7]); // FCMOVNBE ST, ST(7)
    encode32_helper2(Mnemonic::FCMOVNU, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1), &vec![0xDB, 0xD9]); // FCMOVNU ST, ST(1)
}

#[test]
fn instr_fcom() {
    encode32_helper1(Mnemonic::FCOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x10]); // FCOM DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x10]); // FCOM QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOM, Operand::Direct(Reg::ST2), &vec![0xD8, 0xD2]); // FCOM ST(2)
    encode32_helper0(Mnemonic::FCOM, &vec![0xD8, 0xD1]); // FCOM
    encode32_helper1(Mnemonic::FCOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x18]); // FCOMP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x18]); // FCOMP QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FCOMP, Operand::Direct(Reg::ST3), &vec![0xD8, 0xDB]); // FCOMP ST(3)
    encode32_helper1(Mnemonic::FCOMP, Operand::Direct(Reg::ST1), &vec![0xD8, 0xD9]); // FCOMP ST(1)
    encode32_helper0(Mnemonic::FCOMP, &vec![0xD8, 0xD9]); // FCOMP
    encode32_helper0(Mnemonic::FCOMPP, &vec![0xDE, 0xD9]); // FCOMPP 
}

#[test]
fn instr_fcomi() {
    encode32_helper3(Mnemonic::FCOMI, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST4), &vec![0xDB, 0xF4]); // FCOMI ST, ST(4)
    encode32_helper3(Mnemonic::FCOMIP, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST5), &vec![0xDF, 0xF5]); // FCOMIP ST, ST(5)
    encode32_helper3(Mnemonic::FUCOMI, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST6), &vec![0xDB, 0xEE]); // FUCOMI ST, ST(6)
    encode32_helper3(Mnemonic::FUCOMIP, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST7), &vec![0xDF, 0xEF]); // FUCOMIP ST, ST(7)
}

#[test]
fn instr_fcos() {
    encode32_helper0(Mnemonic::FCOS, &vec![0xD9, 0xFF]); // FCOS 
}

#[test]
fn instr_fdecstp() {
    encode32_helper0(Mnemonic::FDECSTP,  &vec![0xD9, 0xF6]); // FDECSTP 
}

#[test]
fn instr_fdiv() {
    // TODO Should probably auto-generate reverse forms as appropriate
    encode32_helper1(Mnemonic::FDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x30]); // FDIV DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x30]); // FDIV QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIV, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xF2]); // FDIV ST, ST(2)
    encode32_helper1(Mnemonic::FIDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x30]); // FIDIV DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x30]); // FIDIV WORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIVP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xF9]); // FDIVP ST(1), ST
}


#[test]
fn instr_fdivr() {
    encode32_helper1(Mnemonic::FDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x38]); // FDIVR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x38]); // FDIVR QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIVR, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST1), &vec![0xD8, 0xF9]); // FDIVR ST, ST(1)
    encode32_helper2(Mnemonic::FDIVR, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xFA]); // FDIVR ST, ST(2)
    encode32_helper1(Mnemonic::FIDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x38]); // FIDIVR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIDIVR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x38]); // FIDIVR WORD PTR [EAX]
    encode32_helper2(Mnemonic::FDIVRP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xF1]); // FDIVRP ST(1), ST
}

#[test]
fn instr_ffree() {
    encode32_helper1(Mnemonic::FFREE, Operand::Direct(Reg::ST4), &vec![0xDD, 0xC4]); // FFREE ST(4)
}

#[test]
fn instr_ficom() {
    encode32_helper1(Mnemonic::FICOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x10]); // FICOM WORD PTR [EAX]
    encode32_helper1(Mnemonic::FICOM, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x10]); // FICOM DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FICOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x18]); // FICOMP WORD PTR [EAX]
    encode32_helper1(Mnemonic::FICOMP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x18]); // FICOMP DWORD PTR [EAX]
}

#[test]
fn instr_fild() {
    encode32_helper1(Mnemonic::FILD, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x00]); // FILD WORD PTR [EAX]
    encode32_helper1(Mnemonic::FILD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x00]); // FILD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FILD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDF, 0x28]); // FILD QWORD PTR [EAX]
}

#[test]
fn instr_fincstp() {
    encode32_helper0(Mnemonic::FINCSTP,  &vec![0xD9, 0xF7]); // FINCSTP 
}

#[test]
fn instr_finit() {
    encode32_helper0(Mnemonic::FINIT,  &vec![0x9B, 0xDB, 0xE3]); // FINIT 
    encode32_helper0(Mnemonic::FNINIT,  &vec![0xDB, 0xE3]); // FNINIT 
}

#[test]
fn instr_fist() {
    encode32_helper1(Mnemonic::FIST, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x10]); // FIST WORD PTR [EAX]
    encode32_helper1(Mnemonic::FIST, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x10]); // FIST DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x18]); // FISTP WORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x18]); // FISTP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDF, 0x38]); // FISTP QWORD PTR [EAX]
}

#[test]
fn instr_fisttp() {
    encode32_helper1(Mnemonic::FISTTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDF, 0x08]); // FISTTP WORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDB, 0x08]); // FISTTP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISTTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x08]); // FISTTP QWORD PTR [EAX]
}

#[test]
fn instr_fld() {
    encode32_helper1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD9, 0x00]); // FLD DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x00]); // FLD QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FLD, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDB, 0x28]); // FLD TBYTE PTR [EAX]
}

#[test]
fn instr_fld_constant() {
    encode32_helper0(Mnemonic::FLD1,  &vec![0xD9, 0xE8]); // FLD1 
    encode32_helper0(Mnemonic::FLDL2T,  &vec![0xD9, 0xE9]); // FLDL2T 
    encode32_helper0(Mnemonic::FLDL2E,  &vec![0xD9, 0xEA]); // FLDL2E 
    encode32_helper0(Mnemonic::FLDPI,  &vec![0xD9, 0xEB]); // FLDPI 
    encode32_helper0(Mnemonic::FLDLG2,  &vec![0xD9, 0xEC]); // FLDLG2 
    encode32_helper0(Mnemonic::FLDLN2,  &vec![0xD9, 0xED]); // FLDLN2 
    encode32_helper0(Mnemonic::FLDZ,  &vec![0xD9, 0xEE]); // FLDZ 
}

#[test]
fn instr_fldcw() {
    encode32_helper1(Mnemonic::FLDCW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xD9, 0x28]); // FLDCW WORD PTR [EAX]
}

#[test]
fn instr_fldenv() {
    encode32_helper1(Mnemonic::FLDENV, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xD9, 0x20]);
}

#[test]
fn instr_fmul() {
    encode32_helper1(Mnemonic::FMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x08]); // FMUL DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x08]); // FMUL QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FMUL, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xCA]); // FMUL ST, ST(2)
    encode32_helper2(Mnemonic::FMUL, Operand::Direct(Reg::ST2), Operand::Direct(Reg::ST), &vec![0xDC, 0xCA]); // FMUL ST(2), ST
    encode32_helper2(Mnemonic::FMULP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xC9]); // FMULP ST(1), ST
    encode32_helper1(Mnemonic::FIMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x08]); // FIMUL DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FIMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x08]); // FIMUL WORD PTR [EAX]
}

#[test]
fn instr_fnop() {
    encode32_helper0(Mnemonic::FNOP,  &vec![0xD9, 0xD0]); // FNOP 
}

#[test]
fn instr_fpatan() {
    encode32_helper0(Mnemonic::FPATAN,  &vec![0xD9, 0xF3]); // FPATAN 
}

#[test]
fn instr_fprem() {
    encode32_helper0(Mnemonic::FPREM,  &vec![0xD9, 0xF8]); // FPREM 
}

#[test]
fn instr_fptan() {
    encode32_helper0(Mnemonic::FPTAN,  &vec![0xD9, 0xF2]); // FPTAN 
}

#[test]
fn instr_frndint() {
    encode32_helper0(Mnemonic::FRNDINT,  &vec![0xD9, 0xFC]); // FRNDINT 
}

#[test]
fn instr_frstor() {
    encode32_helper1(Mnemonic::FRSTOR, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xDD, 0x20]); // FRSTOR [EAX]
}

#[test]
fn instr_fsave() {
    encode32_helper1(Mnemonic::FSAVE, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x9B, 0xDD, 0x30]); // FSAVE [EAX]
    encode32_helper1(Mnemonic::FNSAVE, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xDD, 0x30]); // FNSAVE [EAX]
}

#[test]
fn instr_fscale() {
    encode32_helper0(Mnemonic::FSCALE,  &vec![0xD9, 0xFD]); // FSCALE 
}

#[test]
fn instr_fsin() {
    encode32_helper0(Mnemonic::FSIN,  &vec![0xD9, 0xFE]); // FSIN 
}

#[test]
fn instr_fsincos() {
    encode32_helper0(Mnemonic::FSINCOS,  &vec![0xD9, 0xFB]); // FSINCOS 
}

#[test]
fn instr_fsqrt() {
    encode32_helper0(Mnemonic::FSQRT,  &vec![0xD9, 0xFA]); // FSQRT 
}

#[test]
fn instr_fst() {
    encode32_helper1(Mnemonic::FST, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD9, 0x10]); // FST DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FST, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x10]); // FST QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FST, Operand::Direct(Reg::ST3), &vec![0xDD, 0xD3]); // FST ST(3)
    encode32_helper1(Mnemonic::FSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD9, 0x18]); // FSTP DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDD, 0x18]); // FSTP QWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSTP, Operand::Indirect(Reg::EAX, Some(OperandSize::Tbyte), None), &vec![0xDB, 0x38]); // FSTP TBYTE PTR [EAX]
    encode32_helper1(Mnemonic::FSTP, Operand::Direct(Reg::ST4), &vec![0xDD, 0xDC]); // FSTP ST(4)
}

#[test]
fn instr_fstcw() {
    encode32_helper1(Mnemonic::FSTCW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x9B, 0xD9, 0x38]); // FSTCW WORD PTR [EAX]
    encode32_helper1(Mnemonic::FNSTCW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xD9, 0x38]); // FNSTCW WORD PTR [EAX]
}

#[test]
fn instr_fstenv() {
    encode32_helper1(Mnemonic::FSTENV, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x9B, 0xD9, 0x30]); // FSTENV [EAX]
    encode32_helper1(Mnemonic::FNSTENV, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0xD9, 0x30]); // FNSTENV [EAX]
}

#[test]
fn instr_fstsw() {
    encode32_helper1(Mnemonic::FSTSW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x9B, 0xDD, 0x38]); // FSTSW WORD PTR [EAX]
    encode32_helper1(Mnemonic::FSTSW, Operand::Direct(Reg::AX), &vec![0x9B, 0xDF, 0xE0]); // FSTSW AX
    encode32_helper1(Mnemonic::FNSTSW, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDD, 0x38]); // FNSTSW WORD PTR [EAX]
    encode32_helper1(Mnemonic::FNSTSW, Operand::Direct(Reg::AX), &vec![0xDF, 0xE0]); // FNSTSW AX
}

#[test]
fn instr_fsub() {
    encode32_helper1(Mnemonic::FSUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x20]); // FSUB DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x20]); // FSUB QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FSUB, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST3), &vec![0xD8, 0xE3]); // FSUB ST, ST(3)
    encode32_helper2(Mnemonic::FSUBP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xE9]); // FSUBP ST(1), ST
    encode32_helper1(Mnemonic::FISUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x20]); // FISUB DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISUB, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x20]); // FISUB WORD PTR [EAX]
}

#[test]
fn instr_fsubr() {
    encode32_helper1(Mnemonic::FSUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xD8, 0x28]); // FSUBR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FSUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Qword), None), &vec![0xDC, 0x28]); // FSUBR QWORD PTR [EAX]
    encode32_helper2(Mnemonic::FSUBR, Operand::Direct(Reg::ST), Operand::Direct(Reg::ST2), &vec![0xD8, 0xEA]); // FSUBR ST, ST(2)
    encode32_helper2(Mnemonic::FSUBRP, Operand::Direct(Reg::ST1), Operand::Direct(Reg::ST), &vec![0xDE, 0xE1]); // FSUBRP ST(1), ST
    encode32_helper1(Mnemonic::FISUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xDA, 0x28]); // FISUBR DWORD PTR [EAX]
    encode32_helper1(Mnemonic::FISUBR, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0xDE, 0x28]); // FISUBR WORD PTR [EAX]
}

#[test]
fn instr_ftst() {
    encode32_helper0(Mnemonic::FTST,  &vec![0xD9, 0xE4]); // FTST 
}

#[test]
fn instr_fucom() {
    encode32_helper1(Mnemonic::FUCOM, Operand::Direct(Reg::ST2), &vec![0xDD, 0xE2]); // FUCOM ST(2)
    encode32_helper1(Mnemonic::FUCOM, Operand::Direct(Reg::ST1), &vec![0xDD, 0xE1]); // FUCOM ST(1)
    encode32_helper1(Mnemonic::FUCOMP, Operand::Direct(Reg::ST3), &vec![0xDD, 0xEB]); // FUCOMP ST(3)
    encode32_helper1(Mnemonic::FUCOMP, Operand::Direct(Reg::ST1), &vec![0xDD, 0xE9]); // FUCOMP ST(1)
    encode32_helper0(Mnemonic::FUCOMPP,  &vec![0xDA, 0xE9]); // FUCOMPP 
}

#[test]
fn instr_fxam() {
    encode32_helper0(Mnemonic::FXAM,  &vec![0xD9, 0xE5]); // FXAM 
}

#[test]
fn instr_fxch() {
    encode32_helper1(Mnemonic::FXCH, Operand::Direct(Reg::ST3), &vec![0xD9, 0xCB]); // FXCH ST(3)
    encode32_helper0(Mnemonic::FXCH, &vec![0xD9, 0xC9]); // FXCH
}

#[test]
fn instr_fxrstor() {
    encode32_helper1(Mnemonic::FXRSTOR, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x0F, 0xAE, 0x08]);
}

#[test]
fn instr_fxsave() {
    encode32_helper1(Mnemonic::FXSAVE, Operand::Indirect(Reg::EAX, Some(OperandSize::Unsized), None), &vec![0x0F, 0xAE, 0x00]);
}

#[test]
fn instr_fxtract() {
    encode32_helper0(Mnemonic::FXTRACT,  &vec![0xD9, 0xF4]); // FXTRACT 
}

#[test]
fn instr_fyl2x() {
    encode32_helper0(Mnemonic::FYL2X,  &vec![0xD9, 0xF1]); // FYL2X 
}

#[test]
fn instr_fyl2xp1() {
    encode32_helper0(Mnemonic::FYL2XP1,  &vec![0xD9, 0xF9]); // FYL2XP1 
}

#[test]
fn instr_haddpd() {

}

#[test]
fn instr_haddps() {

}

#[test]
fn instr_hlt() {
    encode32_helper0(Mnemonic::HLT,  &vec![0xF4]); // HLT 
}

#[test]
fn instr_hsubpd() {

}

#[test]
fn instr_hsubps() {

}

#[test]
fn instr_idiv() {
    encode32_helper1(Mnemonic::IDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xF6, 0x38]); // IDIV BYTE PTR [EAX]
    encode32_helper1(Mnemonic::IDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF7, 0x38]); // IDIV WORD PTR [EAX]
    encode32_helper1(Mnemonic::IDIV, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF7, 0x38]); // IDIV DWORD PTR [EAX]
}

#[test]
fn instr_imul() {
    encode32_helper1(Mnemonic::IMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xF6, 0x28]); // IMUL BYTE PTR [EAX]
    encode32_helper1(Mnemonic::IMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xF7, 0x28]); // IMUL WORD PTR [EAX]
    encode32_helper1(Mnemonic::IMUL, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xF7, 0x28]); // IMUL DWORD PTR [EAX]
    encode32_helper2(Mnemonic::IMUL, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0x0F, 0xAF, 0x18]); // IMUL BX, WORD PTR [EAX]
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal8(0x12), &vec![0x66, 0x6B, 0x18, 0x12]); // IMUL BX, WORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal8(0x12), &vec![0x6B, 0x18, 0x12]); // IMUL EBX, DWORD PTR [EAX], 0x12
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::BX), Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), Operand::Literal16(0x1234), &vec![0x66, 0x69, 0x18, 0x34, 0x12]); // IMUL BX, WORD PTR [EAX], 0x1234
    encode32_helper3(Mnemonic::IMUL, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), Operand::Literal32(0x12345678), &vec![0x69, 0x18, 0x78, 0x56, 0x34, 0x12]); // IMUL EBX, DWORD PTR [EAX], 0x12345678
}

#[test]
fn instr_in() {
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AL), Operand::Literal8(0x12), &vec![0xE4, 0x12]); // IN AL, 0x12
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AX), Operand::Literal8(0x12), &vec![0x66, 0xE5, 0x12]); // IN AX, 0x12
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::EAX), Operand::Literal8(0x12), &vec![0xE5, 0x12]); // IN EAX, 0x12
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AL), Operand::Direct(Reg::DX), &vec![0xEC]); // IN AL, DX
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::AX), Operand::Direct(Reg::DX), &vec![0x66, 0xED]); // IN AX, DX
    encode32_helper2(Mnemonic::IN, Operand::Direct(Reg::EAX), Operand::Direct(Reg::DX), &vec![0xED]); // IN EAX, DX
}

#[test]
fn instr_inc() {
    encode32_helper1(Mnemonic::INC, Operand::Indirect(Reg::EAX, Some(OperandSize::Byte), None), &vec![0xFE, 0x00]); // INC BYTE PTR [EAX]
    encode32_helper1(Mnemonic::INC, Operand::Indirect(Reg::EAX, Some(OperandSize::Word), None), &vec![0x66, 0xFF, 0x00]); // INC WORD PTR [EAX]
    encode32_helper1(Mnemonic::INC, Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None), &vec![0xFF, 0x00]); // INC DWORD PTR [EAX]
    encode32_helper1(Mnemonic::INC, Operand::Direct(Reg::BX), &vec![0x66, 0x43]); // INC BX
    encode32_helper1(Mnemonic::INC, Operand::Direct(Reg::EBX), &vec![0x43]); // INC EBX
}

#[test]
fn instr_ins() {
    // TODO Could have these instructions have an optional DX operand, as shown in Intel docs
    encode32_helper0(Mnemonic::INSB, &vec![0x6C]);
    encode32_helper0(Mnemonic::INSW, &vec![0x66, 0x6D]);
    encode32_helper0(Mnemonic::INSD, &vec![0x6D]);
    encode32_helper1(Mnemonic::INS, Operand::Indirect(Reg::EDI, Some(OperandSize::Byte), Some(SegmentReg::ES)), &vec![0x6C]);
    // TODO Add forms of instruction that infer operand size based on memory argument (see Intel
    // docs)
}
