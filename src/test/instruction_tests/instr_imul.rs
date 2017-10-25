use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn imul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: Some(Literal16(20454)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 244, 230, 79], OperandSize::Word)
}

#[test]
fn imul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Memory(32372, Some(OperandSize::Word), None)), operand3: Some(Literal16(5892)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 38, 116, 126, 4, 23], OperandSize::Word)
}

#[test]
fn imul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: Some(Literal16(23469)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 222, 173, 91], OperandSize::Dword)
}

#[test]
fn imul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: Some(Literal16(28669)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 14, 253, 111], OperandSize::Dword)
}

#[test]
fn imul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: Some(Literal16(18794)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 234, 106, 73], OperandSize::Qword)
}

#[test]
fn imul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Word), None)), operand3: Some(Literal16(18095)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 36, 203, 175, 70], OperandSize::Qword)
}

#[test]
fn imul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Literal32(1880619884)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 201, 108, 251, 23, 112], OperandSize::Word)
}

#[test]
fn imul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1099046450)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 56, 50, 30, 130, 65], OperandSize::Word)
}

#[test]
fn imul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: Some(Literal32(1229582769)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 213, 177, 241, 73, 73], OperandSize::Dword)
}

#[test]
fn imul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 2054552175, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1620564667)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 44, 77, 111, 250, 117, 122, 187, 218, 151, 96], OperandSize::Dword)
}

#[test]
fn imul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: Some(Literal32(1203199783)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 251, 39, 95, 183, 71], OperandSize::Qword)
}

#[test]
fn imul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1280728716, Some(OperandSize::Dword), None)), operand3: Some(Literal32(829625643)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 12, 125, 140, 94, 86, 76, 43, 21, 115, 49], OperandSize::Qword)
}

#[test]
fn imul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: Some(Literal32(1318265423)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 219, 79, 34, 147, 78], OperandSize::Qword)
}

#[test]
fn imul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: Some(Literal32(286887089)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 40, 177, 140, 25, 17], OperandSize::Qword)
}

#[test]
fn imul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 213, 97], OperandSize::Word)
}

#[test]
fn imul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(DI, 134, Some(OperandSize::Word), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 189, 134, 0, 113], OperandSize::Word)
}

#[test]
fn imul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 201, 81], OperandSize::Dword)
}

#[test]
fn imul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 2031305432, Some(OperandSize::Word), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 172, 216, 216, 66, 19, 121, 99], OperandSize::Dword)
}

#[test]
fn imul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 215, 52], OperandSize::Qword)
}

#[test]
fn imul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1989516748, Some(OperandSize::Word), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 60, 157, 204, 157, 149, 118, 102], OperandSize::Qword)
}

#[test]
fn imul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 238, 114], OperandSize::Word)
}

#[test]
fn imul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 55, 110], OperandSize::Word)
}

#[test]
fn imul_23() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 228, 35], OperandSize::Dword)
}

#[test]
fn imul_24() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 30, 55], OperandSize::Dword)
}

#[test]
fn imul_25() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 211, 72], OperandSize::Qword)
}

#[test]
fn imul_26() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RBX, 219032679, Some(OperandSize::Dword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 155, 103, 44, 14, 13, 100], OperandSize::Qword)
}

#[test]
fn imul_27() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDI)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 231, 109], OperandSize::Qword)
}

#[test]
fn imul_28() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RDX, 67545481, Some(OperandSize::Qword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 178, 137, 169, 6, 4, 94], OperandSize::Qword)
}

#[test]
fn imul_29() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 231], OperandSize::Word)
}

#[test]
fn imul_30() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 20753, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 177, 17, 81], OperandSize::Word)
}

#[test]
fn imul_31() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 218], OperandSize::Dword)
}

#[test]
fn imul_32() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1189393035, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 28, 253, 139, 178, 228, 70], OperandSize::Dword)
}

#[test]
fn imul_33() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 226], OperandSize::Qword)
}

#[test]
fn imul_34() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 56], OperandSize::Qword)
}

#[test]
fn imul_35() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 217], OperandSize::Word)
}

#[test]
fn imul_36() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 8], OperandSize::Word)
}

#[test]
fn imul_37() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 211], OperandSize::Dword)
}

#[test]
fn imul_38() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 12, 66], OperandSize::Dword)
}

#[test]
fn imul_39() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 234], OperandSize::Qword)
}

#[test]
fn imul_40() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 25], OperandSize::Qword)
}

#[test]
fn imul_41() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 242], OperandSize::Qword)
}

#[test]
fn imul_42() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 601884001, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 28, 221, 97, 5, 224, 35], OperandSize::Qword)
}

#[test]
fn imul_43() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 233], OperandSize::Word)
}

#[test]
fn imul_44() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 40], OperandSize::Word)
}

#[test]
fn imul_45() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 234], OperandSize::Dword)
}

#[test]
fn imul_46() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 44, 194], OperandSize::Dword)
}

#[test]
fn imul_47() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 235], OperandSize::Qword)
}

#[test]
fn imul_48() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(RCX, 1065678012, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 169, 188, 244, 132, 63], OperandSize::Qword)
}

#[test]
fn imul_49() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 233], OperandSize::Word)
}

#[test]
fn imul_50() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(SI, 32648, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 172, 136, 127], OperandSize::Word)
}

#[test]
fn imul_51() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 238], OperandSize::Dword)
}

#[test]
fn imul_52() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 41], OperandSize::Dword)
}

#[test]
fn imul_53() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 236], OperandSize::Qword)
}

#[test]
fn imul_54() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 47], OperandSize::Qword)
}

#[test]
fn imul_55() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 239], OperandSize::Word)
}

#[test]
fn imul_56() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 41], OperandSize::Word)
}

#[test]
fn imul_57() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 236], OperandSize::Dword)
}

#[test]
fn imul_58() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 44, 153], OperandSize::Dword)
}

#[test]
fn imul_59() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 236], OperandSize::Qword)
}

#[test]
fn imul_60() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1055445430, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 44, 197, 182, 209, 232, 62], OperandSize::Qword)
}

#[test]
fn imul_61() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 239], OperandSize::Qword)
}

#[test]
fn imul_62() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 2063355201, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 44, 117, 65, 77, 252, 122], OperandSize::Qword)
}

