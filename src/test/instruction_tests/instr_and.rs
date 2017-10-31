use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn and_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 210], OperandSize::Word)
}

#[test]
fn and_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 12], OperandSize::Word)
}

#[test]
fn and_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 211], OperandSize::Dword)
}

#[test]
fn and_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 12, 134], OperandSize::Dword)
}

#[test]
fn and_5() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 210], OperandSize::Qword)
}

#[test]
fn and_6() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 10], OperandSize::Qword)
}

#[test]
fn and_7() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 202], OperandSize::Qword)
}

#[test]
fn and_8() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 20, 191], OperandSize::Qword)
}

#[test]
fn and_9() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 250], OperandSize::Word)
}

#[test]
fn and_10() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BX, 29600, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 191, 160, 115], OperandSize::Word)
}

#[test]
fn and_11() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 229], OperandSize::Dword)
}

#[test]
fn and_12() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 51], OperandSize::Dword)
}

#[test]
fn and_13() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 238], OperandSize::Qword)
}

#[test]
fn and_14() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 46], OperandSize::Qword)
}

#[test]
fn and_15() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 249], OperandSize::Word)
}

#[test]
fn and_16() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 45], OperandSize::Word)
}

#[test]
fn and_17() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 218], OperandSize::Dword)
}

#[test]
fn and_18() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 36, 190], OperandSize::Dword)
}

#[test]
fn and_19() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 230], OperandSize::Qword)
}

#[test]
fn and_20() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 36, 147], OperandSize::Qword)
}

#[test]
fn and_21() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 249], OperandSize::Qword)
}

#[test]
fn and_22() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 44, 247], OperandSize::Qword)
}

#[test]
fn and_23() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 203], OperandSize::Word)
}

#[test]
fn and_24() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(DI, 157, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 157, 157, 0], OperandSize::Word)
}

#[test]
fn and_25() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 211], OperandSize::Dword)
}

#[test]
fn and_26() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 12, 137], OperandSize::Dword)
}

#[test]
fn and_27() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 219], OperandSize::Qword)
}

#[test]
fn and_28() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1639806262, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 156, 113, 54, 117, 189, 97], OperandSize::Qword)
}

#[test]
fn and_29() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 201], OperandSize::Qword)
}

#[test]
fn and_30() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 16], OperandSize::Qword)
}

#[test]
fn and_31() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 231], OperandSize::Word)
}

#[test]
fn and_32() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Memory(6008, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 54, 120, 23], OperandSize::Word)
}

#[test]
fn and_33() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 233], OperandSize::Dword)
}

#[test]
fn and_34() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1149748011, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 140, 199, 43, 195, 135, 68], OperandSize::Dword)
}

#[test]
fn and_35() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 206], OperandSize::Qword)
}

#[test]
fn and_36() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 52, 146], OperandSize::Qword)
}

#[test]
fn and_37() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 234], OperandSize::Word)
}

#[test]
fn and_38() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 30826, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 155, 106, 120], OperandSize::Word)
}

#[test]
fn and_39() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 231], OperandSize::Dword)
}

#[test]
fn and_40() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1780039499, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 52, 253, 75, 63, 25, 106], OperandSize::Dword)
}

#[test]
fn and_41() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 205], OperandSize::Qword)
}

#[test]
fn and_42() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1251098190, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 164, 78, 78, 62, 146, 74], OperandSize::Qword)
}

#[test]
fn and_43() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 238], OperandSize::Qword)
}

#[test]
fn and_44() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 35, 36, 75], OperandSize::Qword)
}

#[test]
fn and_45() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 108], OperandSize::Word)
}

#[test]
fn and_46() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 43], OperandSize::Dword)
}

#[test]
fn and_47() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 33], OperandSize::Qword)
}

#[test]
fn and_48() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(26712)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 88, 104], OperandSize::Word)
}

#[test]
fn and_49() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(26008)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 152, 101], OperandSize::Dword)
}

#[test]
fn and_50() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(11346)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 82, 44], OperandSize::Qword)
}

#[test]
fn and_51() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1353446294)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 150, 243, 171, 80], OperandSize::Word)
}

#[test]
fn and_52() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1454792220)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 28, 94, 182, 86], OperandSize::Dword)
}

#[test]
fn and_53() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(199812800)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 192, 230, 232, 11], OperandSize::Qword)
}

#[test]
fn and_54() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RAX)), operand2: Some(Literal32(2099265585)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 37, 49, 64, 32, 125], OperandSize::Qword)
}

#[test]
fn and_55() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 88], OperandSize::Word)
}

#[test]
fn and_56() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 35, 57], OperandSize::Word)
}

#[test]
fn and_57() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 11], OperandSize::Dword)
}

#[test]
fn and_58() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 223, 12], OperandSize::Dword)
}

#[test]
fn and_59() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 24], OperandSize::Qword)
}

#[test]
fn and_60() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1949483121, Some(OperandSize::Byte), None)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 213, 113, 192, 50, 116, 34], OperandSize::Qword)
}

#[test]
fn and_61() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 226, 105], OperandSize::Qword)
}

#[test]
fn and_62() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 616231446, Some(OperandSize::Byte), None)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 164, 177, 22, 242, 186, 36, 39], OperandSize::Qword)
}

#[test]
fn and_63() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(Literal16(14954)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 228, 106, 58], OperandSize::Word)
}

#[test]
fn and_64() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(SI, 175, Some(OperandSize::Word), None)), operand2: Some(Literal16(7656)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 164, 175, 0, 232, 29], OperandSize::Word)
}

#[test]
fn and_65() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Literal16(31127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 225, 151, 121], OperandSize::Dword)
}

#[test]
fn and_66() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(31282)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 36, 179, 50, 122], OperandSize::Dword)
}

#[test]
fn and_67() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Literal16(6973)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 225, 61, 27], OperandSize::Qword)
}

#[test]
fn and_68() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 81553018, Some(OperandSize::Word), None)), operand2: Some(Literal16(19373)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 164, 64, 122, 102, 220, 4, 173, 75], OperandSize::Qword)
}

#[test]
fn and_69() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(Literal32(675381048)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 225, 56, 127, 65, 40], OperandSize::Word)
}

#[test]
fn and_70() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BX, 68, Some(OperandSize::Dword), None)), operand2: Some(Literal32(544893355)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 103, 68, 171, 105, 122, 32], OperandSize::Word)
}

#[test]
fn and_71() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal32(610372459)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 227, 107, 139, 97, 36], OperandSize::Dword)
}

#[test]
fn and_72() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1964504151)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 32, 87, 244, 23, 117], OperandSize::Dword)
}

#[test]
fn and_73() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Literal32(948040022)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 231, 86, 241, 129, 56], OperandSize::Qword)
}

#[test]
fn and_74() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RAX, 1331012440, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1515276432)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 160, 88, 163, 85, 79, 144, 72, 81, 90], OperandSize::Qword)
}

#[test]
fn and_75() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBP)), operand2: Some(Literal32(825372822)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 229, 150, 48, 50, 49], OperandSize::Qword)
}

#[test]
fn and_76() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1809234559)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 39, 127, 186, 214, 107], OperandSize::Qword)
}

#[test]
fn and_77() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 226, 109], OperandSize::Word)
}

#[test]
fn and_78() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 33, 2], OperandSize::Word)
}

#[test]
fn and_79() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 226, 49], OperandSize::Dword)
}

#[test]
fn and_80() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 35, 44], OperandSize::Dword)
}

#[test]
fn and_81() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 226, 4], OperandSize::Qword)
}

#[test]
fn and_82() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 36, 223, 57], OperandSize::Qword)
}

#[test]
fn and_83() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 226, 16], OperandSize::Word)
}

#[test]
fn and_84() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 34, 89], OperandSize::Word)
}

#[test]
fn and_85() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 228, 12], OperandSize::Dword)
}

#[test]
fn and_86() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 34, 105], OperandSize::Dword)
}

#[test]
fn and_87() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 226, 14], OperandSize::Qword)
}

#[test]
fn and_88() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RDI, 419489698, Some(OperandSize::Dword), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 167, 162, 231, 0, 25, 126], OperandSize::Qword)
}

#[test]
fn and_89() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBX)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 227, 21], OperandSize::Qword)
}

#[test]
fn and_90() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1575755568, Some(OperandSize::Qword), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 36, 141, 48, 31, 236, 93, 61], OperandSize::Qword)
}

