use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 89, 242], OperandSize::Dword)
}

#[test]
fn vmulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 89, 62], OperandSize::Dword)
}

#[test]
fn vmulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 89, 210], OperandSize::Qword)
}

#[test]
fn vmulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 89, 20, 178], OperandSize::Qword)
}

#[test]
fn vmulsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 207, 189, 89, 248], OperandSize::Dword)
}

#[test]
fn vmulsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 1244027579, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 215, 137, 89, 135, 187, 90, 38, 74], OperandSize::Dword)
}

#[test]
fn vmulsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 255, 251, 89, 212], OperandSize::Qword)
}

#[test]
fn vmulsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 215, 135, 89, 4, 136], OperandSize::Qword)
}

