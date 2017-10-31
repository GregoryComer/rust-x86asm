use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 157, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 531765083, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 157, 12, 205, 91, 23, 178, 31], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 157, 224], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 157, 60, 203], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 218, 157, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1348734217, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 143, 157, 178, 9, 13, 100, 80], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 189, 181, 157, 247], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 135, 157, 25], OperandSize::Qword)
}

