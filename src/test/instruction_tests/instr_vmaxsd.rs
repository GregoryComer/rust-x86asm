use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 95, 254], OperandSize::Dword)
}

#[test]
fn vmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 299041325, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 95, 191, 45, 2, 211, 17], OperandSize::Dword)
}

#[test]
fn vmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 95, 245], OperandSize::Qword)
}

#[test]
fn vmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 258160519, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 95, 20, 221, 135, 55, 99, 15], OperandSize::Qword)
}

#[test]
fn vmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 247, 156, 95, 252], OperandSize::Dword)
}

#[test]
fn vmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 683711948, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 247, 139, 95, 184, 204, 157, 192, 40], OperandSize::Dword)
}

#[test]
fn vmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 191, 148, 95, 240], OperandSize::Qword)
}

#[test]
fn vmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 183, 139, 95, 20, 88], OperandSize::Qword)
}

