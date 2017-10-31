use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn add_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Word)
}

#[test]
fn add_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 187, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 140, 187, 0], OperandSize::Word)
}

#[test]
fn add_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 219], OperandSize::Dword)
}

#[test]
fn add_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 28, 219], OperandSize::Dword)
}

#[test]
fn add_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 219], OperandSize::Qword)
}

#[test]
fn add_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RDX, 895350478, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 138, 206, 246, 93, 53], OperandSize::Qword)
}

#[test]
fn add_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 203], OperandSize::Qword)
}

#[test]
fn add_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RDX, Two, 508910508, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 20, 85, 172, 91, 85, 30], OperandSize::Qword)
}

#[test]
fn add_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 212], OperandSize::Word)
}

#[test]
fn add_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 32, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 105, 32], OperandSize::Word)
}

#[test]
fn add_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 222], OperandSize::Dword)
}

#[test]
fn add_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 798365205, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 140, 216, 21, 22, 150, 47], OperandSize::Dword)
}

#[test]
fn add_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 215], OperandSize::Qword)
}

#[test]
fn add_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RBX, 431345417, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 187, 9, 207, 181, 25], OperandSize::Qword)
}

#[test]
fn add_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 254], OperandSize::Word)
}

#[test]
fn add_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 67, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 99, 67], OperandSize::Word)
}

#[test]
fn add_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 237], OperandSize::Dword)
}

#[test]
fn add_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1630291436, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 172, 209, 236, 69, 44, 97], OperandSize::Dword)
}

#[test]
fn add_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 217], OperandSize::Qword)
}

#[test]
fn add_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RBX, 2039473766, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 147, 102, 230, 143, 121], OperandSize::Qword)
}

#[test]
fn add_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 209], OperandSize::Qword)
}

#[test]
fn add_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 60, 153], OperandSize::Qword)
}

#[test]
fn add_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 202], OperandSize::Word)
}

#[test]
fn add_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 70, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 88, 70], OperandSize::Word)
}

#[test]
fn add_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Dword)
}

#[test]
fn add_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 129956690, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 148, 177, 82, 251, 190, 7], OperandSize::Dword)
}

#[test]
fn add_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 201], OperandSize::Qword)
}

#[test]
fn add_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 12, 130], OperandSize::Qword)
}

#[test]
fn add_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Qword)
}

#[test]
fn add_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 28, 206], OperandSize::Qword)
}

#[test]
fn add_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 212], OperandSize::Word)
}

#[test]
fn add_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 235, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 147, 235, 0], OperandSize::Word)
}

#[test]
fn add_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 242], OperandSize::Dword)
}

#[test]
fn add_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 27], OperandSize::Dword)
}

#[test]
fn add_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 246], OperandSize::Qword)
}

#[test]
fn add_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1977935466, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 140, 88, 106, 230, 228, 117], OperandSize::Qword)
}

#[test]
fn add_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 227], OperandSize::Word)
}

#[test]
fn add_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 31248, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 171, 16, 122], OperandSize::Word)
}

#[test]
fn add_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 210], OperandSize::Dword)
}

#[test]
fn add_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EAX, 246028574, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 184, 30, 25, 170, 14], OperandSize::Dword)
}

#[test]
fn add_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 239], OperandSize::Qword)
}

#[test]
fn add_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RDX, 840891874, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 170, 226, 253, 30, 50], OperandSize::Qword)
}

#[test]
fn add_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 235], OperandSize::Qword)
}

#[test]
fn add_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1756198842, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 3, 188, 147, 186, 119, 173, 104], OperandSize::Qword)
}

#[test]
fn add_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 117], OperandSize::Word)
}

#[test]
fn add_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 35], OperandSize::Dword)
}

#[test]
fn add_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 11], OperandSize::Qword)
}

#[test]
fn add_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(29635)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 195, 115], OperandSize::Word)
}

#[test]
fn add_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(7850)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 170, 30], OperandSize::Dword)
}

#[test]
fn add_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(31688)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 200, 123], OperandSize::Qword)
}

#[test]
fn add_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(80079763)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 147, 235, 197, 4], OperandSize::Word)
}

#[test]
fn add_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1031859061)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 117, 235, 128, 61], OperandSize::Dword)
}

#[test]
fn add_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1060432008)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 136, 232, 52, 63], OperandSize::Qword)
}

#[test]
fn add_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1177385117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 5, 157, 120, 45, 70], OperandSize::Qword)
}

#[test]
fn add_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 3], OperandSize::Word)
}

#[test]
fn add_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 4, 9], OperandSize::Word)
}

#[test]
fn add_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 195, 66], OperandSize::Dword)
}

#[test]
fn add_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 0, 92], OperandSize::Dword)
}

#[test]
fn add_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 52], OperandSize::Qword)
}

#[test]
fn add_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RSI, 2117975325, Some(OperandSize::Byte), None)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 134, 29, 189, 61, 126, 42], OperandSize::Qword)
}

#[test]
fn add_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 116], OperandSize::Qword)
}

#[test]
fn add_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1749860578, Some(OperandSize::Byte), None)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 121, 226, 192, 76, 104, 99], OperandSize::Qword)
}

#[test]
fn add_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Literal16(21025)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 198, 33, 82], OperandSize::Word)
}

#[test]
fn add_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(BP, 0, Some(OperandSize::Word), None)), operand2: Some(Literal16(13258)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 70, 0, 202, 51], OperandSize::Word)
}

#[test]
fn add_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Literal16(22357)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 197, 85, 87], OperandSize::Dword)
}

#[test]
fn add_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(19041)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 4, 89, 97, 74], OperandSize::Dword)
}

#[test]
fn add_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Literal16(7779)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 196, 99, 30], OperandSize::Qword)
}

#[test]
fn add_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RCX, 768681918, Some(OperandSize::Word), None)), operand2: Some(Literal16(30363)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 129, 190, 39, 209, 45, 155, 118], OperandSize::Qword)
}

#[test]
fn add_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal32(868714884)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 197, 132, 137, 199, 51], OperandSize::Word)
}

#[test]
fn add_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(999843964)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 2, 124, 104, 152, 59], OperandSize::Word)
}

#[test]
fn add_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1589149460)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 197, 20, 127, 184, 94], OperandSize::Dword)
}

#[test]
fn add_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(465637451)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 1, 75, 16, 193, 27], OperandSize::Dword)
}

#[test]
fn add_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1029652821)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 197, 85, 65, 95, 61], OperandSize::Qword)
}

#[test]
fn add_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1233279544, Some(OperandSize::Dword), None)), operand2: Some(Literal32(718352558)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 4, 205, 56, 90, 130, 73, 174, 48, 209, 42], OperandSize::Qword)
}

#[test]
fn add_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDI)), operand2: Some(Literal32(755822562)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 199, 226, 239, 12, 45], OperandSize::Qword)
}

#[test]
fn add_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Literal32(860867837)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 6, 253, 204, 79, 51], OperandSize::Qword)
}

#[test]
fn add_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 195, 103], OperandSize::Word)
}

#[test]
fn add_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 174, Some(OperandSize::Word), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 132, 174, 0, 75], OperandSize::Word)
}

#[test]
fn add_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 194, 34], OperandSize::Dword)
}

#[test]
fn add_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1122555346, Some(OperandSize::Word), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 132, 215, 210, 213, 232, 66, 115], OperandSize::Dword)
}

#[test]
fn add_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 199, 27], OperandSize::Qword)
}

#[test]
fn add_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 4, 207, 55], OperandSize::Qword)
}

#[test]
fn add_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 194, 109], OperandSize::Word)
}

#[test]
fn add_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 93, Some(OperandSize::Dword), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 64, 93, 93], OperandSize::Word)
}

#[test]
fn add_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 197, 52], OperandSize::Dword)
}

#[test]
fn add_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(EAX, 398121705, Some(OperandSize::Dword), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 128, 233, 218, 186, 23, 36], OperandSize::Dword)
}

#[test]
fn add_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 198, 47], OperandSize::Qword)
}

#[test]
fn add_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1527590690, Some(OperandSize::Dword), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 4, 125, 34, 47, 13, 91, 126], OperandSize::Qword)
}

#[test]
fn add_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBP)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 197, 64], OperandSize::Qword)
}

#[test]
fn add_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1695626993, Some(OperandSize::Qword), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 4, 205, 241, 54, 17, 101, 101], OperandSize::Qword)
}

