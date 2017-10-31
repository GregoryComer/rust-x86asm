use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 89, 200], OperandSize::Dword)
}

#[test]
fn vmulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 89, 36, 70], OperandSize::Dword)
}

#[test]
fn vmulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 89, 204], OperandSize::Qword)
}

#[test]
fn vmulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1435994099, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 89, 148, 70, 243, 135, 151, 85], OperandSize::Qword)
}

#[test]
fn vmulsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 215, 185, 89, 244], OperandSize::Dword)
}

#[test]
fn vmulsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 239, 138, 89, 7], OperandSize::Dword)
}

#[test]
fn vmulsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 191, 189, 89, 192], OperandSize::Qword)
}

#[test]
fn vmulsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RSI, 427911220, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 207, 138, 89, 190, 52, 104, 129, 25], OperandSize::Qword)
}

