use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 94, 235], OperandSize::Dword)
}

#[test]
fn vdivsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 94, 4, 81], OperandSize::Dword)
}

#[test]
fn vdivsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 94, 206], OperandSize::Qword)
}

#[test]
fn vdivsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 94, 60, 80], OperandSize::Qword)
}

#[test]
fn vdivsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 215, 190, 94, 249], OperandSize::Dword)
}

#[test]
fn vdivsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 215, 137, 94, 4, 187], OperandSize::Dword)
}

#[test]
fn vdivsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 191, 217, 94, 250], OperandSize::Qword)
}

#[test]
fn vdivsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RDX, 2030388784, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 135, 143, 94, 154, 48, 70, 5, 121], OperandSize::Qword)
}

