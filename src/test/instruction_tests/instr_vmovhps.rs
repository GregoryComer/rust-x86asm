use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 22, 7], OperandSize::Dword)
}

#[test]
fn vmovhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 22, 16], OperandSize::Qword)
}

#[test]
fn vmovhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1021046465, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 22, 12, 85, 193, 238, 219, 60], OperandSize::Dword)
}

#[test]
fn vmovhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1782341751, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 100, 0, 22, 156, 86, 119, 96, 60, 106], OperandSize::Qword)
}

#[test]
fn vmovhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 457686137, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 148, 75, 121, 188, 71, 27], OperandSize::Dword)
}

#[test]
fn vmovhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectDisplaced(RDI, 1262935843, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 175, 35, 223, 70, 75], OperandSize::Qword)
}

#[test]
fn vmovhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledDisplaced(EDX, Four, 257407565, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 52, 149, 77, 186, 87, 15], OperandSize::Dword)
}

#[test]
fn vmovhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 17], OperandSize::Qword)
}

