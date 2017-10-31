use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 157, 223], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 358450327, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 157, 154, 151, 132, 93, 21], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 157, 205], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 668332428, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 157, 131, 140, 241, 213, 39], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 254, 157, 219], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 157, 28, 95], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 101, 254, 157, 248], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RAX, 769418769, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 93, 129, 157, 160, 17, 102, 220, 45], OperandSize::Qword)
}

