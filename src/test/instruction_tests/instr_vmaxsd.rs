use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 95, 219], OperandSize::Dword)
}

#[test]
fn vmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1377660995, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 95, 4, 141, 67, 112, 29, 82], OperandSize::Dword)
}

#[test]
fn vmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 95, 236], OperandSize::Qword)
}

#[test]
fn vmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 4490773, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 95, 131, 21, 134, 68, 0], OperandSize::Qword)
}

#[test]
fn vmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 231, 156, 95, 232], OperandSize::Dword)
}

#[test]
fn vmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1798608735, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 223, 142, 95, 153, 95, 151, 52, 107], OperandSize::Dword)
}

#[test]
fn vmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 255, 145, 95, 244], OperandSize::Qword)
}

#[test]
fn vmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 223, 134, 95, 32], OperandSize::Qword)
}

