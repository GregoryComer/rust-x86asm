use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 157, 218], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 157, 51], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 157, 220], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 51500506, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 157, 136, 218, 213, 17, 3], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 250, 157, 218], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 157, 36, 155], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 249, 157, 214], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 117, 135, 157, 28, 206], OperandSize::Qword)
}

