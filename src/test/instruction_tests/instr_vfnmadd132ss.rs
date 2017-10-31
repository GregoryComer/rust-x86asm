use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 157, 221], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 157, 12, 134], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 157, 243], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 157, 44, 240], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 254, 157, 253], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 157, 0], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 37, 221, 157, 222], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RDI, 920724029, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 77, 131, 157, 135, 61, 34, 225, 54], OperandSize::Qword)
}

