use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 229], OperandSize::Dword)
}

#[test]
fn vmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 60, 94], OperandSize::Dword)
}

#[test]
fn vmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 236], OperandSize::Qword)
}

#[test]
fn vmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 1011472962, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 183, 66, 218, 73, 60], OperandSize::Qword)
}

#[test]
fn vmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 220], OperandSize::Dword)
}

#[test]
fn vmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 1934520677, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 150, 101, 113, 78, 115], OperandSize::Dword)
}

#[test]
fn vmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 239], OperandSize::Qword)
}

#[test]
fn vmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM11)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 110, 27], OperandSize::Qword)
}

#[test]
fn vmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 230], OperandSize::Dword)
}

#[test]
fn vmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 800948521, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 140, 126, 41, 129, 189, 47], OperandSize::Dword)
}

#[test]
fn vmovd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 209], OperandSize::Qword)
}

#[test]
fn vmovd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 20, 88], OperandSize::Qword)
}

#[test]
fn vmovd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 243], OperandSize::Dword)
}

#[test]
fn vmovd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(EDI, 16258387, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 191, 83, 21, 248, 0], OperandSize::Dword)
}

#[test]
fn vmovd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 8, 126, 213], OperandSize::Qword)
}

#[test]
fn vmovd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(RDI, 631889873, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 8, 126, 151, 209, 223, 169, 37], OperandSize::Qword)
}

