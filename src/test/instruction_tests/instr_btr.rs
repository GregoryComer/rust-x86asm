use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 245], OperandSize::Word)
}

#[test]
fn btr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 118, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 91, 118], OperandSize::Word)
}

#[test]
fn btr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 207], OperandSize::Dword)
}

#[test]
fn btr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1550808265, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 20, 77, 201, 116, 111, 92], OperandSize::Dword)
}

#[test]
fn btr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 205], OperandSize::Qword)
}

#[test]
fn btr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 52, 89], OperandSize::Qword)
}

#[test]
fn btr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 226], OperandSize::Word)
}

#[test]
fn btr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 91, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 122, 91], OperandSize::Word)
}

#[test]
fn btr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 215], OperandSize::Dword)
}

#[test]
fn btr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 48], OperandSize::Dword)
}

#[test]
fn btr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 231], OperandSize::Qword)
}

#[test]
fn btr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 39], OperandSize::Qword)
}

#[test]
fn btr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 207], OperandSize::Qword)
}

#[test]
fn btr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 44, 80], OperandSize::Qword)
}

#[test]
fn btr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(SI)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 246, 109], OperandSize::Word)
}

#[test]
fn btr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 6800, Some(OperandSize::Word), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 177, 144, 26, 63], OperandSize::Word)
}

#[test]
fn btr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(CX)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 241, 62], OperandSize::Dword)
}

#[test]
fn btr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(EAX, 237031303, Some(OperandSize::Word), None)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 176, 135, 207, 32, 14, 79], OperandSize::Dword)
}

#[test]
fn btr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BP)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 245, 6], OperandSize::Qword)
}

#[test]
fn btr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1992152250, Some(OperandSize::Word), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 180, 81, 186, 212, 189, 118, 112], OperandSize::Qword)
}

#[test]
fn btr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 246, 18], OperandSize::Word)
}

#[test]
fn btr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(DI, 29217, Some(OperandSize::Dword), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 181, 33, 114, 48], OperandSize::Word)
}

#[test]
fn btr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 242, 92], OperandSize::Dword)
}

#[test]
fn btr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 142, 4], OperandSize::Dword)
}

#[test]
fn btr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 245, 97], OperandSize::Qword)
}

#[test]
fn btr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RDX, 1582033998, Some(OperandSize::Dword), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 178, 78, 236, 75, 94, 7], OperandSize::Qword)
}

#[test]
fn btr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 245, 6], OperandSize::Qword)
}

#[test]
fn btr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 52, 241, 85], OperandSize::Qword)
}

