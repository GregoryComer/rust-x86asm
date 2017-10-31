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
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 9], OperandSize::Word)
}

#[test]
fn xor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 210], OperandSize::Dword)
}

#[test]
fn xor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDI, 636151536, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 151, 240, 230, 234, 37], OperandSize::Dword)
}

#[test]
fn xor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 209], OperandSize::Qword)
}

#[test]
fn xor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 369730194, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 140, 195, 146, 162, 9, 22], OperandSize::Qword)
}

#[test]
fn xor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Qword)
}

#[test]
fn xor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 658416568, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 20, 85, 184, 163, 62, 39], OperandSize::Qword)
}

#[test]
fn xor_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 252], OperandSize::Word)
}

#[test]
fn xor_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 55], OperandSize::Word)
}

#[test]
fn xor_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 215], OperandSize::Dword)
}

#[test]
fn xor_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1121927979, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 156, 131, 43, 67, 223, 66], OperandSize::Dword)
}

#[test]
fn xor_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 221], OperandSize::Qword)
}

#[test]
fn xor_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 185154186, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 44, 133, 138, 58, 9, 11], OperandSize::Qword)
}

#[test]
fn xor_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 203], OperandSize::Word)
}

#[test]
fn xor_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 178, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 145, 178, 0], OperandSize::Word)
}

#[test]
fn xor_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 207], OperandSize::Dword)
}

#[test]
fn xor_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 52, 203], OperandSize::Dword)
}

#[test]
fn xor_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 225], OperandSize::Qword)
}

#[test]
fn xor_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 9], OperandSize::Qword)
}

#[test]
fn xor_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 210], OperandSize::Qword)
}

#[test]
fn xor_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RAX, 327735996, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 144, 188, 218, 136, 19], OperandSize::Qword)
}

#[test]
fn xor_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 201], OperandSize::Word)
}

#[test]
fn xor_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 8], OperandSize::Word)
}

#[test]
fn xor_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 218], OperandSize::Dword)
}

#[test]
fn xor_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(ESI, 533713080, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 142, 184, 208, 207, 31], OperandSize::Dword)
}

#[test]
fn xor_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 201], OperandSize::Qword)
}

#[test]
fn xor_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 16], OperandSize::Qword)
}

#[test]
fn xor_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 218], OperandSize::Qword)
}

#[test]
fn xor_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RBX, 187314217, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 147, 41, 48, 42, 11], OperandSize::Qword)
}

#[test]
fn xor_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 228], OperandSize::Word)
}

#[test]
fn xor_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 35], OperandSize::Word)
}

#[test]
fn xor_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 238], OperandSize::Dword)
}

#[test]
fn xor_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 31], OperandSize::Dword)
}

#[test]
fn xor_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 229], OperandSize::Qword)
}

#[test]
fn xor_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(RDX, 1551659861, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 154, 85, 115, 124, 92], OperandSize::Qword)
}

#[test]
fn xor_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 205], OperandSize::Word)
}

#[test]
fn xor_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BP, 196, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 190, 196, 0], OperandSize::Word)
}

#[test]
fn xor_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 210], OperandSize::Dword)
}

#[test]
fn xor_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 335063362, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 20, 93, 66, 169, 248, 19], OperandSize::Dword)
}

#[test]
fn xor_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 254], OperandSize::Qword)
}

#[test]
fn xor_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1225078321, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 20, 117, 49, 54, 5, 73], OperandSize::Qword)
}

#[test]
fn xor_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 202], OperandSize::Qword)
}

#[test]
fn xor_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 852859334, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 51, 20, 133, 198, 153, 213, 50], OperandSize::Qword)
}

#[test]
fn xor_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 86], OperandSize::Word)
}

#[test]
fn xor_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 17], OperandSize::Dword)
}

#[test]
fn xor_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 53], OperandSize::Qword)
}

#[test]
fn xor_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(7794)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 114, 30], OperandSize::Word)
}

#[test]
fn xor_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(30275)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 67, 118], OperandSize::Dword)
}

#[test]
fn xor_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(19373)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 173, 75], OperandSize::Qword)
}

#[test]
fn xor_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(260394367)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 127, 77, 133, 15], OperandSize::Word)
}

#[test]
fn xor_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1835543574)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 22, 44, 104, 109], OperandSize::Dword)
}

#[test]
fn xor_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(715454494)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 30, 248, 164, 42], OperandSize::Qword)
}

#[test]
fn xor_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1376280883)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 53, 51, 97, 8, 82], OperandSize::Qword)
}

#[test]
fn xor_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 89], OperandSize::Word)
}

#[test]
fn xor_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(SI, 24, Some(OperandSize::Byte), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 116, 24, 17], OperandSize::Word)
}

#[test]
fn xor_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 91], OperandSize::Dword)
}

#[test]
fn xor_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1023955470, Some(OperandSize::Byte), None)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 180, 113, 14, 82, 8, 61, 114], OperandSize::Dword)
}

#[test]
fn xor_59() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 67], OperandSize::Qword)
}

#[test]
fn xor_60() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1703718053, Some(OperandSize::Byte), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 180, 203, 165, 172, 140, 101, 65], OperandSize::Qword)
}

#[test]
fn xor_61() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 81], OperandSize::Qword)
}

#[test]
fn xor_62() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 48, 113], OperandSize::Qword)
}

#[test]
fn xor_63() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Literal16(4470)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 245, 118, 17], OperandSize::Word)
}

#[test]
fn xor_64() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(DI, 205, Some(OperandSize::Word), None)), operand2: Some(Literal16(21699)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 181, 205, 0, 195, 84], OperandSize::Word)
}

#[test]
fn xor_65() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Literal16(8458)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 245, 10, 33], OperandSize::Dword)
}

#[test]
fn xor_66() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDI, 1254418475, Some(OperandSize::Word), None)), operand2: Some(Literal16(19746)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 183, 43, 232, 196, 74, 34, 77], OperandSize::Dword)
}

#[test]
fn xor_67() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal16(30925)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 247, 205, 120], OperandSize::Qword)
}

#[test]
fn xor_68() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 2122306720, Some(OperandSize::Word), None)), operand2: Some(Literal16(8486)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 245, 160, 212, 127, 126, 38, 33], OperandSize::Qword)
}

#[test]
fn xor_69() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Literal32(446577940)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 241, 20, 61, 158, 26], OperandSize::Word)
}

#[test]
fn xor_70() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Memory(4289, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1017806999)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 54, 193, 16, 151, 128, 170, 60], OperandSize::Word)
}

#[test]
fn xor_71() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Literal32(911388404)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 246, 244, 174, 82, 54], OperandSize::Dword)
}

#[test]
fn xor_72() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(589007389)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 49, 29, 138, 27, 35], OperandSize::Dword)
}

#[test]
fn xor_73() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1701777657)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 245, 249, 16, 111, 101], OperandSize::Qword)
}

#[test]
fn xor_74() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(764497429)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 50, 21, 78, 145, 45], OperandSize::Qword)
}

#[test]
fn xor_75() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RSP)), operand2: Some(Literal32(52518626)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 244, 226, 94, 33, 3], OperandSize::Qword)
}

#[test]
fn xor_76() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RDX, 356344376, Some(OperandSize::Qword), None)), operand2: Some(Literal32(978948448)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 178, 56, 98, 61, 21, 96, 145, 89, 58], OperandSize::Qword)
}

#[test]
fn xor_77() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 246, 8], OperandSize::Word)
}

#[test]
fn xor_78() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(DI, 21, Some(OperandSize::Word), None)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 117, 21, 79], OperandSize::Word)
}

#[test]
fn xor_79() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 243, 116], OperandSize::Dword)
}

#[test]
fn xor_80() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDI, 1688825600, Some(OperandSize::Word), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 183, 0, 111, 169, 100, 23], OperandSize::Dword)
}

#[test]
fn xor_81() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 244, 23], OperandSize::Qword)
}

#[test]
fn xor_82() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 52, 185, 105], OperandSize::Qword)
}

#[test]
fn xor_83() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 244, 121], OperandSize::Word)
}

#[test]
fn xor_84() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 59, Some(OperandSize::Dword), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 113, 59, 63], OperandSize::Word)
}

#[test]
fn xor_85() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 246, 102], OperandSize::Dword)
}

#[test]
fn xor_86() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 52, 201, 111], OperandSize::Dword)
}

#[test]
fn xor_87() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 247, 84], OperandSize::Qword)
}

#[test]
fn xor_88() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1876032225, Some(OperandSize::Dword), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 52, 93, 225, 250, 209, 111, 103], OperandSize::Qword)
}

#[test]
fn xor_89() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 245, 120], OperandSize::Qword)
}

#[test]
fn xor_90() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 55, 75], OperandSize::Qword)
}

