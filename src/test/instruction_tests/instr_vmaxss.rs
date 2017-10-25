use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 95, 250], OperandSize::Dword)
}

#[test]
fn vmaxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1683877442, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 95, 20, 93, 66, 238, 93, 100], OperandSize::Dword)
}

#[test]
fn vmaxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 95, 236], OperandSize::Qword)
}

#[test]
fn vmaxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 387701623, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 95, 52, 93, 119, 219, 27, 23], OperandSize::Qword)
}

#[test]
fn vmaxss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 118, 155, 95, 251], OperandSize::Dword)
}

#[test]
fn vmaxss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 95, 12, 138], OperandSize::Dword)
}

#[test]
fn vmaxss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 70, 148, 95, 214], OperandSize::Qword)
}

#[test]
fn vmaxss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1921255896, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 54, 139, 95, 4, 69, 216, 9, 132, 114], OperandSize::Qword)
}

