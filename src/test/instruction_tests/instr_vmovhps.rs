use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 424704493, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 22, 148, 145, 237, 121, 80, 25], OperandSize::Dword)
}

#[test]
fn vmovhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 1614185482, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 22, 176, 10, 132, 54, 96], OperandSize::Qword)
}

#[test]
fn vmovhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 430793889, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 22, 180, 215, 161, 100, 173, 25], OperandSize::Dword)
}

#[test]
fn vmovhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 60, 0, 22, 20, 187], OperandSize::Qword)
}

#[test]
fn vmovhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectDisplaced(ESI, 1705439393, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 182, 161, 240, 166, 101], OperandSize::Dword)
}

#[test]
fn vmovhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1640562423, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 52, 77, 247, 254, 200, 97], OperandSize::Qword)
}

#[test]
fn vmovhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 44, 186], OperandSize::Dword)
}

#[test]
fn vmovhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 23, 19], OperandSize::Qword)
}

