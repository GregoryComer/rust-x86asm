use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 231], OperandSize::Word)
}

#[test]
fn btr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 10448, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 187, 208, 40], OperandSize::Word)
}

#[test]
fn btr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 230], OperandSize::Dword)
}

#[test]
fn btr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 24], OperandSize::Dword)
}

#[test]
fn btr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 210], OperandSize::Qword)
}

#[test]
fn btr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1647619756, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 52, 133, 172, 174, 52, 98], OperandSize::Qword)
}

#[test]
fn btr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 210], OperandSize::Word)
}

#[test]
fn btr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Memory(11028, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 62, 20, 43], OperandSize::Word)
}

#[test]
fn btr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 249], OperandSize::Dword)
}

#[test]
fn btr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1565809884, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 60, 181, 220, 92, 84, 93], OperandSize::Dword)
}

#[test]
fn btr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 206], OperandSize::Qword)
}

#[test]
fn btr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1760307746, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 44, 157, 34, 42, 236, 104], OperandSize::Qword)
}

#[test]
fn btr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 211], OperandSize::Qword)
}

#[test]
fn btr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 2056758218, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 52, 221, 202, 163, 151, 122], OperandSize::Qword)
}

#[test]
fn btr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DX)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 242, 23], OperandSize::Word)
}

#[test]
fn btr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(SI, 27582, Some(OperandSize::Word), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 180, 190, 107, 17], OperandSize::Word)
}

#[test]
fn btr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BX)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 243, 12], OperandSize::Dword)
}

#[test]
fn btr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 52, 120, 82], OperandSize::Dword)
}

#[test]
fn btr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(CX)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 241, 80], OperandSize::Qword)
}

#[test]
fn btr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RSI, 904959889, Some(OperandSize::Word), None)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 182, 145, 151, 240, 53, 72], OperandSize::Qword)
}

#[test]
fn btr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 244, 120], OperandSize::Word)
}

#[test]
fn btr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 18245, Some(OperandSize::Dword), None)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 178, 69, 71, 95], OperandSize::Word)
}

#[test]
fn btr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 243, 100], OperandSize::Dword)
}

#[test]
fn btr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(EDX, Four, 168808494, Some(OperandSize::Dword), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 149, 46, 208, 15, 10, 14], OperandSize::Dword)
}

#[test]
fn btr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 243, 97], OperandSize::Qword)
}

#[test]
fn btr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 946698728, Some(OperandSize::Dword), None)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 85, 232, 121, 109, 56, 90], OperandSize::Qword)
}

#[test]
fn btr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 242, 113], OperandSize::Qword)
}

#[test]
fn btr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RBX, 306800380, Some(OperandSize::Qword), None)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 179, 252, 102, 73, 18, 40], OperandSize::Qword)
}

