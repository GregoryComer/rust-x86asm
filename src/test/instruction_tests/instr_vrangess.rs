use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 155, 81, 245, 55], OperandSize::Dword)
}

#[test]
fn vrangess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 137, 81, 43, 76], OperandSize::Dword)
}

#[test]
fn vrangess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM21)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 37, 145, 81, 213, 4], OperandSize::Qword)
}

#[test]
fn vrangess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 910523040, Some(OperandSize::Dword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 109, 139, 81, 172, 206, 160, 122, 69, 54, 121], OperandSize::Qword)
}

