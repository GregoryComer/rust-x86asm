use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 219], OperandSize::Word)
}

#[test]
fn xor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 19], OperandSize::Word)
}

#[test]
fn xor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 217], OperandSize::Dword)
}

#[test]
fn xor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDI, 677023413, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 151, 181, 142, 90, 40], OperandSize::Dword)
}

#[test]
fn xor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 210], OperandSize::Qword)
}

#[test]
fn xor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RAX, 2089624097, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 136, 33, 34, 141, 124], OperandSize::Qword)
}

#[test]
fn xor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 219], OperandSize::Qword)
}

#[test]
fn xor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 505896887, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 148, 89, 183, 95, 39, 30], OperandSize::Qword)
}

#[test]
fn xor_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 212], OperandSize::Word)
}

#[test]
fn xor_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 15312, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 145, 208, 59], OperandSize::Word)
}

#[test]
fn xor_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 223], OperandSize::Dword)
}

#[test]
fn xor_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1398401143, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 172, 202, 119, 232, 89, 83], OperandSize::Dword)
}

#[test]
fn xor_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 213], OperandSize::Qword)
}

#[test]
fn xor_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 39], OperandSize::Qword)
}

#[test]
fn xor_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 202], OperandSize::Word)
}

#[test]
fn xor_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 19439, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 146, 239, 75], OperandSize::Word)
}

#[test]
fn xor_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 254], OperandSize::Dword)
}

#[test]
fn xor_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EBX, 359280693, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 139, 53, 48, 106, 21], OperandSize::Dword)
}

#[test]
fn xor_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 215], OperandSize::Qword)
}

#[test]
fn xor_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 246874344, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 12, 213, 232, 0, 183, 14], OperandSize::Qword)
}

#[test]
fn xor_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 215], OperandSize::Qword)
}

#[test]
fn xor_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 324748828, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 188, 219, 28, 70, 91, 19], OperandSize::Qword)
}

#[test]
fn xor_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 202], OperandSize::Word)
}

#[test]
fn xor_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 13], OperandSize::Word)
}

#[test]
fn xor_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Dword)
}

#[test]
fn xor_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 407712087, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 28, 253, 87, 49, 77, 24], OperandSize::Dword)
}

#[test]
fn xor_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 210], OperandSize::Qword)
}

#[test]
fn xor_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RSI, 1876764229, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 150, 69, 38, 221, 111], OperandSize::Qword)
}

#[test]
fn xor_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 217], OperandSize::Qword)
}

#[test]
fn xor_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1173619947, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 156, 177, 235, 4, 244, 69], OperandSize::Qword)
}

#[test]
fn xor_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 218], OperandSize::Word)
}

#[test]
fn xor_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BP, 26941, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 190, 61, 105], OperandSize::Word)
}

#[test]
fn xor_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 254], OperandSize::Dword)
}

#[test]
fn xor_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1243156585, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 60, 133, 105, 16, 25, 74], OperandSize::Dword)
}

#[test]
fn xor_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 220], OperandSize::Qword)
}

#[test]
fn xor_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 62], OperandSize::Qword)
}

#[test]
fn xor_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 209], OperandSize::Word)
}

#[test]
fn xor_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 29], OperandSize::Word)
}

#[test]
fn xor_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 205], OperandSize::Dword)
}

#[test]
fn xor_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 28, 78], OperandSize::Dword)
}

#[test]
fn xor_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 211], OperandSize::Qword)
}

#[test]
fn xor_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 52, 150], OperandSize::Qword)
}

#[test]
fn xor_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 225], OperandSize::Qword)
}

#[test]
fn xor_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 316815258, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 51, 44, 85, 154, 55, 226, 18], OperandSize::Qword)
}

#[test]
fn xor_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 21], OperandSize::Word)
}

#[test]
fn xor_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 53], OperandSize::Dword)
}

#[test]
fn xor_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 91], OperandSize::Qword)
}

#[test]
fn xor_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(26331)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 219, 102], OperandSize::Word)
}

#[test]
fn xor_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(16270)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 142, 63], OperandSize::Dword)
}

#[test]
fn xor_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(23889)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 81, 93], OperandSize::Qword)
}

#[test]
fn xor_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1805274003)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 147, 75, 154, 107], OperandSize::Word)
}

#[test]
fn xor_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1041681159)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 7, 203, 22, 62], OperandSize::Dword)
}

#[test]
fn xor_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1267929830)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 230, 18, 147, 75], OperandSize::Qword)
}

#[test]
fn xor_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(945223845)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 53, 165, 248, 86, 56], OperandSize::Qword)
}

#[test]
fn xor_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 15], OperandSize::Word)
}

#[test]
fn xor_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(DI, 10247, Some(OperandSize::Byte), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 181, 7, 40, 24], OperandSize::Word)
}

#[test]
fn xor_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 243, 43], OperandSize::Dword)
}

#[test]
fn xor_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 48, 69], OperandSize::Dword)
}

#[test]
fn xor_59() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 241, 74], OperandSize::Qword)
}

#[test]
fn xor_60() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 49, 73], OperandSize::Qword)
}

#[test]
fn xor_61() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 241, 69], OperandSize::Qword)
}

#[test]
fn xor_62() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 50, 82], OperandSize::Qword)
}

#[test]
fn xor_63() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Literal16(6381)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 245, 237, 24], OperandSize::Word)
}

#[test]
fn xor_64() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 13408, Some(OperandSize::Word), None)), operand2: Some(Literal16(19293)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 178, 96, 52, 93, 75], OperandSize::Word)
}

#[test]
fn xor_65() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DX)), operand2: Some(Literal16(2819)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 242, 3, 11], OperandSize::Dword)
}

#[test]
fn xor_66() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1789044380, Some(OperandSize::Word), None)), operand2: Some(Literal16(22931)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 77, 156, 166, 162, 106, 147, 89], OperandSize::Dword)
}

#[test]
fn xor_67() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Literal16(23370)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 245, 74, 91], OperandSize::Qword)
}

#[test]
fn xor_68() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RDI, 1664532250, Some(OperandSize::Word), None)), operand2: Some(Literal16(21413)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 183, 26, 191, 54, 99, 165, 83], OperandSize::Qword)
}

#[test]
fn xor_69() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Literal32(1395010885)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 244, 69, 45, 38, 83], OperandSize::Word)
}

#[test]
fn xor_70() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1519013490)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 55, 114, 78, 138, 90], OperandSize::Word)
}

#[test]
fn xor_71() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(Literal32(293461673)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 247, 169, 222, 125, 17], OperandSize::Dword)
}

#[test]
fn xor_72() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDX, 193403094, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1949635665)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 178, 214, 24, 135, 11, 81, 20, 53, 116], OperandSize::Dword)
}

#[test]
fn xor_73() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1102733510)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 243, 198, 96, 186, 65], OperandSize::Qword)
}

#[test]
fn xor_74() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal32(2091808427)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 52, 121, 171, 118, 174, 124], OperandSize::Qword)
}

#[test]
fn xor_75() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RBX)), operand2: Some(Literal32(1687339239)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 243, 231, 192, 146, 100], OperandSize::Qword)
}

#[test]
fn xor_76() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RCX, Four, 21077301, Some(OperandSize::Qword), None)), operand2: Some(Literal32(503865901)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 52, 141, 53, 157, 65, 1, 45, 98, 8, 30], OperandSize::Qword)
}

#[test]
fn xor_77() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 246, 47], OperandSize::Word)
}

#[test]
fn xor_78() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(SI, 135, Some(OperandSize::Word), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 180, 135, 0, 67], OperandSize::Word)
}

#[test]
fn xor_79() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CX)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 241, 32], OperandSize::Dword)
}

#[test]
fn xor_80() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1775151431, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 180, 216, 71, 169, 206, 105, 113], OperandSize::Dword)
}

#[test]
fn xor_81() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DX)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 242, 67], OperandSize::Qword)
}

#[test]
fn xor_82() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1516682258, Some(OperandSize::Word), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 52, 213, 18, 188, 102, 90, 31], OperandSize::Qword)
}

#[test]
fn xor_83() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 245, 77], OperandSize::Word)
}

#[test]
fn xor_84() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 49, 53], OperandSize::Word)
}

#[test]
fn xor_85() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 241, 104], OperandSize::Dword)
}

#[test]
fn xor_86() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(ECX, 1520700690, Some(OperandSize::Dword), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 177, 18, 13, 164, 90, 86], OperandSize::Dword)
}

#[test]
fn xor_87() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 244, 44], OperandSize::Qword)
}

#[test]
fn xor_88() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 55, 25], OperandSize::Qword)
}

#[test]
fn xor_89() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 243, 9], OperandSize::Qword)
}

#[test]
fn xor_90() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 52, 122, 54], OperandSize::Qword)
}

