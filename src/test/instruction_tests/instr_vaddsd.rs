use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 88, 238], OperandSize::Dword)
}

#[test]
fn vaddsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1631186556, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 88, 142, 124, 238, 57, 97], OperandSize::Dword)
}

#[test]
fn vaddsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 88, 248], OperandSize::Qword)
}

#[test]
fn vaddsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 2138788345, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 88, 28, 197, 249, 81, 123, 127], OperandSize::Qword)
}

#[test]
fn vaddsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 207, 253, 88, 232], OperandSize::Dword)
}

#[test]
fn vaddsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 215, 137, 88, 52, 151], OperandSize::Dword)
}

#[test]
fn vaddsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 223, 253, 88, 239], OperandSize::Qword)
}

#[test]
fn vaddsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 135, 139, 88, 63], OperandSize::Qword)
}

