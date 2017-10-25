use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 153, 225], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 153, 46], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 153, 202], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 153, 19], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 158, 153, 222], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 189262361, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 153, 172, 216, 25, 234, 71, 11], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 13, 180, 153, 206], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 29, 131, 153, 28, 73], OperandSize::Qword)
}

