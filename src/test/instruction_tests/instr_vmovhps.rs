use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 856063963, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 22, 136, 219, 127, 6, 51], OperandSize::Dword)
}

#[test]
fn vmovhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 22, 20, 128], OperandSize::Qword)
}

#[test]
fn vmovhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 22, 60, 201], OperandSize::Dword)
}

#[test]
fn vmovhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 200935738, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 4, 0, 22, 60, 205, 58, 9, 250, 11], OperandSize::Qword)
}

#[test]
fn vmovhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1102152140, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 28, 93, 204, 129, 177, 65], OperandSize::Dword)
}

#[test]
fn vmovhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 36, 183], OperandSize::Qword)
}

#[test]
fn vmovhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1531337588, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 12, 133, 116, 91, 70, 91], OperandSize::Dword)
}

#[test]
fn vmovhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 59], OperandSize::Qword)
}

